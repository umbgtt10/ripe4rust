// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::command_output::CommandOutput;
use crate::system::model::invocation::Invocation;
use crate::system::model::launch_failure::LaunchFailure;
use crate::system::traits::command_runner::CommandRunner;
use std::io::ErrorKind;
use std::process::Command;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread::scope;

const AT_ONCE: usize = 8;

/// Runs a program as a child process, its output captured, and without
/// colour, so what cargo and cargo-deny print reads as recorded; a batch of
/// them at most eight at once, each on a thread of its own, answered in the
/// batch's order.
pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, LaunchFailure> {
        let output = Command::new(&invocation.program)
            .args(&invocation.args)
            .current_dir(&invocation.dir)
            .env("CARGO_TERM_COLOR", "never")
            .output()
            .map_err(|error| match error.kind() {
                ErrorKind::NotFound => LaunchFailure::Missing {
                    program: invocation.program.clone(),
                },
                _ => LaunchFailure::Failed {
                    program: invocation.program.clone(),
                    detail: error.to_string(),
                },
            })?;
        Ok(CommandOutput {
            status: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }

    fn run_all(&self, invocations: &[Invocation]) -> Vec<Result<CommandOutput, LaunchFailure>> {
        let next = AtomicUsize::new(0);
        let mut answered = scope(|scope| {
            let workers = (0..AT_ONCE.min(invocations.len()))
                .map(|_| scope.spawn(|| self.worker(invocations, &next)))
                .collect::<Vec<_>>();
            workers
                .into_iter()
                .flat_map(|worker| worker.join().unwrap_or_default())
                .collect::<Vec<(usize, Result<CommandOutput, LaunchFailure>)>>()
        });
        answered.sort_by_key(|(at, _)| *at);
        let mut answered = answered.into_iter().peekable();
        invocations
            .iter()
            .enumerate()
            .map(
                |(at, invocation)| match answered.next_if(|(done, _)| *done == at) {
                    Some((_, ran)) => ran,
                    None => Err(LaunchFailure::Failed {
                        program: invocation.program.clone(),
                        detail: String::from("the thread running it stopped"),
                    }),
                },
            )
            .collect()
    }
}

impl SystemCommandRunner {
    fn worker(
        &self,
        invocations: &[Invocation],
        next: &AtomicUsize,
    ) -> Vec<(usize, Result<CommandOutput, LaunchFailure>)> {
        let mut done = Vec::new();
        loop {
            let at = next.fetch_add(1, Ordering::Relaxed);
            let Some(invocation) = invocations.get(at) else {
                return done;
            };
            done.push((at, self.run(invocation)));
        }
    }
}

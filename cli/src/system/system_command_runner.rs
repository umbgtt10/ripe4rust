// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::seams::command_output::CommandOutput;
use crate::seams::invocation::Invocation;
use crate::seams::launch_failure::LaunchFailure;
use crate::traits::command_runner::CommandRunner;
use std::io::ErrorKind;
use std::process::Command;

/// Runs a program as a child process, its output captured, and without
/// colour, so what cargo and cargo-deny print reads as recorded.
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
}

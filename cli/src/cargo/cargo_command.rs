// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cargo::complaint::Complaint;
use crate::seams::command_output::CommandOutput;
use crate::seams::invocation::Invocation;
use crate::traits::command_runner::CommandRunner;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

const CARGO: &str = "cargo";

/// One cargo command in the workspace's directory, which must succeed: one
/// that cannot start, or ends in failure, is the failure its adapter names,
/// with what cargo said on one line.
pub struct CargoCommand<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
}

impl<'a> CargoCommand<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf) -> Self {
        Self { runner, dir }
    }

    pub fn run(
        &self,
        args: Vec<String>,
        failed: fn(String) -> PortFailure,
    ) -> Result<CommandOutput, PortFailure> {
        let output = self
            .runner
            .run(&Invocation {
                program: String::from(CARGO),
                args,
                dir: self.dir.clone(),
            })
            .map_err(|failure| failed(Complaint.unlaunched(&failure)))?;
        if output.status == Some(0) {
            Ok(output)
        } else {
            Err(failed(Complaint.of(&output)))
        }
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::command_output::CommandOutput;
use crate::system::model::invocation::Invocation;
use crate::system::model::launch_failure::LaunchFailure;

/// Runs a program to its end and hands back what it printed: every
/// adapter's process goes through it, so each is tested on recorded output.
pub trait CommandRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, LaunchFailure>;
}

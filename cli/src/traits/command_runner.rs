// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::seams::command_output::CommandOutput;
use crate::seams::invocation::Invocation;
use crate::seams::launch_failure::LaunchFailure;

/// Runs a program to its end and hands back what it printed: every
/// adapter's process goes through it, so each is tested on recorded output.
pub trait CommandRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, LaunchFailure>;
}

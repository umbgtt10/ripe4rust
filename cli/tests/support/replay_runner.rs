// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::model::command_output::CommandOutput;
use cargo_ripe4rust::system::model::invocation::Invocation;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use cargo_ripe4rust::system::traits::command_runner::CommandRunner;
use std::cell::RefCell;

/// A runner that replays recorded outputs, each for the command line it was
/// recorded for, and keeps every invocation it was asked to run.
pub struct ReplayRunner {
    pub replies: Vec<(Vec<String>, Result<CommandOutput, LaunchFailure>)>,
    pub calls: RefCell<Vec<Invocation>>,
}

impl ReplayRunner {
    pub fn new() -> Self {
        Self {
            replies: Vec::new(),
            calls: RefCell::new(Vec::new()),
        }
    }

    pub fn commands(&self) -> Vec<String> {
        self.calls
            .borrow()
            .iter()
            .map(|call| {
                let mut words = vec![call.program.clone()];
                words.extend(call.args.iter().cloned());
                words.join(" ")
            })
            .collect()
    }

    pub fn refuse(self, command: &[impl AsRef<str>], failure: LaunchFailure) -> Self {
        self.answer(command, Err(failure))
    }

    pub fn reply(self, command: &[impl AsRef<str>], output: CommandOutput) -> Self {
        self.answer(command, Ok(output))
    }

    fn answer(
        mut self,
        command: &[impl AsRef<str>],
        output: Result<CommandOutput, LaunchFailure>,
    ) -> Self {
        self.replies.push((
            command
                .iter()
                .map(|word| String::from(word.as_ref()))
                .collect(),
            output,
        ));
        self
    }
}

impl CommandRunner for ReplayRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, LaunchFailure> {
        self.calls.borrow_mut().push(invocation.clone());
        let mut asked = vec![invocation.program.clone()];
        asked.extend(invocation.args.iter().cloned());
        self.replies
            .iter()
            .find(|(command, _)| *command == asked)
            .map_or_else(
                || panic!("no recording for `{}`", asked.join(" ")),
                |(_, output)| output.clone(),
            )
    }
}

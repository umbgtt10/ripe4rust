// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::answer::Answer;
use crate::command::check_command::CheckCommand;
use crate::command::command_line::CommandLine;
use crate::command::hold_command::HoldCommand;
use crate::command::plan_command::PlanCommand;
use crate::command::ran::Ran;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::guiding::guide::Guide;

const CHECK: &str = "check";
const GUIDE: &str = "guide";
const HOLD: &str = "hold";
const RELEASE: &str = "--version";
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The library's entry point, which `main` is a thin shim over: the
/// command line read, the command it names run over the seams it was given,
/// and the one document that command prints with the code it exits with --
/// `--version` the binary and its release, exit 0; a command line refused is
/// its diagnostic, exit 2.
pub struct Ripe4rust<'a> {
    seams: Seams<'a>,
}

impl<'a> Ripe4rust<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn run(&self, args: &[String]) -> Ran {
        let request = match CommandLine.read(args) {
            Ok(request) => request,
            Err(refusal) => {
                return Answer::new(refusal.format, &refusal.command).failed(&PlanFailure {
                    diagnostics: vec![refusal.diagnostic],
                });
            }
        };
        let answer = Answer::new(request.format, &request.command);
        let seams = self.seams.clone();
        match request.command.as_str() {
            CHECK => answer.checked(CheckCommand::new(seams).check(&request)),
            GUIDE => answer.guided(&Guide::new(VERSION).text()),
            HOLD => answer.held(HoldCommand::new(seams).hold(&request)),
            RELEASE => answer.versioned(VERSION),
            _ => answer.planned(PlanCommand::new(seams).plan(&request)),
        }
    }
}

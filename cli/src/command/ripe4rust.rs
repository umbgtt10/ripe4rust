// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::command_line::CommandLine;
use crate::command::document::Document;
use crate::command::plan_command::PlanCommand;
use crate::command::ran::Ran;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan_failure::PlanFailure;

const PLANNED: u8 = 0;
const NO_PLAN: u8 = 2;

/// The library's entry point, which `main` is a thin shim over: the
/// command line read, the command run over the seams it was given, and the
/// one document it prints -- 0 where a plan was produced, whatever it says,
/// 2 where none could be.
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
                return Ran {
                    output: Document::new(refusal.format, &refusal.command).failure(&PlanFailure {
                        diagnostics: vec![refusal.diagnostic],
                    }),
                    code: NO_PLAN,
                };
            }
        };
        let document = Document::new(request.format, &request.command);
        match PlanCommand::new(self.seams.clone()).plan(&request) {
            Ok(plan) => Ran {
                output: document.plan(&plan),
                code: PLANNED,
            },
            Err(failure) => Ran {
                output: document.failure(&failure),
                code: NO_PLAN,
            },
        }
    }
}

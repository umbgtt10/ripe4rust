// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::document::Document;
use crate::command::exits::Exits;
use crate::command::format::Format;
use crate::command::ran::Ran;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

const PLANNED: u8 = 0;

/// What a command answers, in the format its line asked for: its result
/// with the code it exits with -- 0 for a plan, whatever it says; 0 or 1 for
/// a check --, or, where it could not do its work, the failure, exit 3 for a
/// stale plan and 2 otherwise.
pub struct Answer {
    document: Document,
}

impl Answer {
    pub fn new(format: Format, command: &str) -> Self {
        Self {
            document: Document::new(format, command),
        }
    }

    pub fn checked(&self, result: Result<CheckReport, PlanFailure>) -> Ran {
        self.of(result, |report| Ran {
            output: self.document.check(report),
            code: Exits.check(report),
        })
    }

    pub fn failed(&self, failure: &PlanFailure) -> Ran {
        Ran {
            output: self.document.failure(failure),
            code: Exits.failure(failure),
        }
    }

    pub fn planned(&self, result: Result<Plan, PlanFailure>) -> Ran {
        self.of(result, |plan| Ran {
            output: self.document.plan(plan),
            code: PLANNED,
        })
    }

    fn of<T>(&self, result: Result<T, PlanFailure>, printed: impl Fn(&T) -> Ran) -> Ran {
        result.map_or_else(|failure| self.failed(&failure), |value| printed(&value))
    }
}

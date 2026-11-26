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
use ripe4rust_core::planning::round::guiding::guide_text::GuideText;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;

const PLANNED: u8 = 0;
const PRINTED: u8 = 0;

/// What a command answers, in the format its line asked for: its result
/// with the code it exits with -- 0 for a plan, whatever it says; 0 or 1 for
/// a check; 0 for the pins a hold prints, for the guide and for the binary's
/// release --, or, where it could not do its
/// work, the failure, exit 3 for a stale plan and 2 otherwise.
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
        self.of(result, Document::check, |report| Exits.check(report))
    }

    pub fn failed(&self, failure: &PlanFailure) -> Ran {
        Ran {
            output: self.document.failure(failure),
            code: Exits.failure(failure),
        }
    }

    pub fn guided(&self, text: &GuideText) -> Ran {
        Ran {
            output: self.document.guide(text),
            code: PRINTED,
        }
    }

    pub fn held(&self, result: Result<HoldReport, PlanFailure>) -> Ran {
        self.of(result, Document::hold, |_| PRINTED)
    }

    pub fn planned(&self, result: Result<Plan, PlanFailure>) -> Ran {
        self.of(result, Document::plan, |_| PLANNED)
    }

    pub fn versioned(&self, version: &str) -> Ran {
        Ran {
            output: format!("cargo-ripe4rust {version}\n"),
            code: PRINTED,
        }
    }

    fn of<T>(
        &self,
        result: Result<T, PlanFailure>,
        print: fn(&Document, &T) -> String,
        code: impl Fn(&T) -> u8,
    ) -> Ran {
        match result {
            Ok(value) => Ran {
                output: print(&self.document, &value),
                code: code(&value),
            },
            Err(failure) => self.failed(&failure),
        }
    }
}

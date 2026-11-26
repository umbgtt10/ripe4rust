// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

const AS_PLANNED: u8 = 0;
const NOT_AS_PLANNED: u8 = 1;
const COULD_NOT: u8 = 2;
const STALE: u8 = 3;

/// The code a command exits with: `check`'s 0 for a unit as planned and 1
/// for one that is not; for a command that could not do its work, 3 where
/// the plan is stale and 2 otherwise.
pub struct Exits;

impl Exits {
    pub fn check(&self, report: &CheckReport) -> u8 {
        if report.as_planned {
            AS_PLANNED
        } else {
            NOT_AS_PLANNED
        }
    }

    pub fn failure(&self, failure: &PlanFailure) -> u8 {
        if failure
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == DiagnosticCode::PlanStale)
        {
            STALE
        } else {
            COULD_NOT
        }
    }
}

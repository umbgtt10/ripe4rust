// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::lock_move::LockMove;
use crate::planning::round::line_change::LineChange;
use crate::planning::round::move_wording::MoveWording;
use crate::planning::writing::constants::PLAN_FILE;
use crate::values::location::Location;
use alloc::format;
use alloc::string::String;

/// A change outside the plan, each a `plan-stale` diagnostic where it is:
/// plan again, and go on with the new plan.
pub struct StaleFindings;

impl StaleFindings {
    pub fn line(&self, change: &LineChange) -> Diagnostic {
        let (at, what) = match change {
            LineChange::Gone { then } => (
                &then.at,
                format!(
                    "`{}` at {}, which read `{}`, is gone, and no unit of the plan removes it",
                    then.key, then.at, then.value
                ),
            ),
            LineChange::New { now } => (
                &now.at,
                format!(
                    "`{}` at {} reads `{}`, a line the plan did not find, and no unit of the plan writes it",
                    now.key, now.at, now.value
                ),
            ),
            LineChange::Rewritten { then, now } => (
                &now.at,
                format!(
                    "`{}` at {} reads `{}`, where the plan found `{}`, and no unit of the plan writes it",
                    now.key, now.at, now.value, then.value
                ),
            ),
        };
        Self::stale(Some(at.clone()), what)
    }

    pub fn lock(&self, lock_move: &LockMove) -> Diagnostic {
        Self::stale(
            None,
            format!("in `Cargo.lock`, {}", MoveWording.unexplained(lock_move)),
        )
    }

    fn stale(at: Option<Location>, what: String) -> Diagnostic {
        DiagnosticCode::PlanStale.diagnose(
            at,
            what,
            format!(
                "plan again -- `cargo ripe4rust plan --format json > {PLAN_FILE}` -- and go on with the new plan"
            ),
        )
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::planning::plan_failure::PlanFailure;
use crate::planning::round::holding::held_crates::HeldCrates;
use crate::planning::round::holding::hold_report::HoldReport;
use crate::planning::round::holding::pin_line::PinLine;
use crate::planning::round::holding::pin_placement::PinPlacement;
use crate::planning::round::holding::pin_writer::PinWriter;
use crate::planning::round::round_reader::RoundReader;
use crate::planning::round::sorting::Sorting;
use crate::planning::round::stale_findings::StaleFindings;
use crate::planning::round::target_finder::TargetFinder;
use crate::traits::plan_source::PlanSource;
use crate::traits::workspace_files::WorkspaceFiles;
use crate::values::date::Date;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const SECTION: &str = "[workspace.metadata.ripe4rust.pins]";

/// `hold`: the plan read, the unit named found in it -- an urgent one is never
/// held --, the workspace read again and sorted as `check` sorts it, and the
/// pins that hold the unit, with where they go. With no step taken, any change
/// no unit explains, in the lock too, is the plan stale -- but a change of a
/// crate the units share, which is theirs together; the review date is the
/// one given, or `hold-days` after the plan's date.
pub struct Holder<'a> {
    plan: &'a dyn PlanSource,
    files: &'a dyn WorkspaceFiles,
}

impl<'a> Holder<'a> {
    pub fn new(plan: &'a dyn PlanSource, files: &'a dyn WorkspaceFiles) -> Self {
        Self { plan, files }
    }

    pub fn hold(
        &self,
        unit: &str,
        because: &str,
        review: Option<Date>,
    ) -> Result<HoldReport, PlanFailure> {
        let refused = |diagnostic: Diagnostic| PlanFailure {
            diagnostics: vec![diagnostic],
        };
        let plan = self
            .plan
            .plan()
            .map_err(|failure| refused(failure.diagnostic()))?;
        let target = TargetFinder::new(&plan).find(unit).map_err(refused)?;
        if target.unit.urgent {
            return Err(refused(DiagnosticCode::UnitUrgent.diagnose(
                None,
                format!(
                    "`{unit}` is urgent, and an urgent unit is never held: holding it would keep the advisory or the yanked release it clears"
                ),
                String::from(
                    "do not hold it: it becomes an urgent ask -- carry out its `revert` if it is applied, and put its question to the person, with the failure attached, ahead of every other",
                ),
            )));
        }
        let inputs = RoundReader::new(self.files).read(&plan)?;
        let sorted = Sorting::new(&plan, &inputs.current).sort();
        let mut stale = sorted.stale;
        if sorted.breaking.is_empty() {
            stale.extend(
                sorted
                    .unexplained
                    .iter()
                    .filter(|lock_move| !sorted.shared.contains(lock_move))
                    .map(|lock_move| StaleFindings.lock(lock_move)),
            );
        }
        stale.extend(
            sorted
                .unexplained_git
                .iter()
                .map(|moved| StaleFindings.git(moved)),
        );
        if !stale.is_empty() {
            let mut diagnostics = inputs.diagnostics;
            diagnostics.extend(stale);
            return Err(PlanFailure { diagnostics });
        }
        let writer = PinWriter::new(
            because,
            review.unwrap_or_else(|| {
                plan.date
                    .add_days(i64::from(inputs.policy.thresholds.hold_days))
            }),
        );
        let pins = HeldCrates
            .of(&target)
            .iter()
            .map(|held| {
                let pinned = inputs.policy.pins.iter().find(|pin| pin.name == held.name);
                PinLine {
                    name: held.name.clone(),
                    line: writer.line(held, pinned),
                    replaces: pinned.map(|pin| pin.at.clone()),
                }
            })
            .collect::<Vec<PinLine>>();
        Ok(HoldReport {
            unit: target.id,
            section: String::from(SECTION),
            action: PinPlacement.action(inputs.pins_at.as_ref(), &pins),
            at: inputs.pins_at,
            pins,
            diagnostics: inputs.diagnostics,
        })
    }
}

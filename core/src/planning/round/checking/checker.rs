// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::planning::plan_failure::PlanFailure;
use crate::planning::round::checking::check_report::CheckReport;
use crate::planning::round::checking::difference_finder::DifferenceFinder;
use crate::planning::round::round_reader::RoundReader;
use crate::planning::round::sorting::Sorting;
use crate::planning::round::target_finder::TargetFinder;
use crate::traits::plan_source::PlanSource;
use crate::traits::workspace_files::WorkspaceFiles;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec;

/// `check`: the plan read, the unit named found in it, the workspace read
/// again and every difference sorted. A change no unit explains is the plan
/// stale; otherwise the unit is as planned or not, every difference with
/// what to do.
pub struct Checker<'a> {
    plan: &'a dyn PlanSource,
    files: &'a dyn WorkspaceFiles,
}

impl<'a> Checker<'a> {
    pub fn new(plan: &'a dyn PlanSource, files: &'a dyn WorkspaceFiles) -> Self {
        Self { plan, files }
    }

    pub fn check(&self, unit: &str) -> Result<CheckReport, PlanFailure> {
        let refused = |diagnostic: Diagnostic| PlanFailure {
            diagnostics: vec![diagnostic],
        };
        let plan = self
            .plan
            .plan()
            .map_err(|failure| refused(failure.diagnostic()))?;
        let target = TargetFinder::new(&plan).find(unit).map_err(refused)?;
        if target.unit.kind == UnitKind::Advisory {
            return Err(refused(DiagnosticCode::UnitUnknown.diagnose(
                None,
                format!(
                    "`{unit}` is an advisory row: it applies nothing, so there is nothing to check"
                ),
                String::from(
                    "put its question to the person: an advisory row is answered, never applied",
                ),
            )));
        }
        let inputs = RoundReader::new(self.files).read(&plan)?;
        let sorting = Sorting::new(&plan, &inputs.current);
        let sorted = sorting.sort();
        if !sorted.stale.is_empty() {
            let mut diagnostics = inputs.diagnostics;
            diagnostics.extend(sorted.stale);
            return Err(PlanFailure { diagnostics });
        }
        let finder = DifferenceFinder::new(&target, &sorting, &inputs.policy.pins);
        let differences = finder.differences(&sorted);
        Ok(CheckReport {
            unit: target.id.clone(),
            as_planned: differences.is_empty(),
            transitive: finder.transitive(&sorted),
            differences,
            applied: sorted.applied,
            to_go: sorted.to_go,
            diagnostics: inputs.diagnostics,
        })
    }
}

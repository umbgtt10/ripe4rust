// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::document_renderer::DocumentRenderer;
use crate::render::table::check_table::CheckTable;
use crate::render::table::columns::Columns;
use crate::render::table::diagnostic_line::DiagnosticLine;
use crate::render::table::guide_table::GuideTable;
use crate::render::table::hold_table::HoldTable;
use crate::render::table::row_label::RowLabel;
use crate::render::table::target_cell::TargetCell;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::guiding::guide_text::GuideText;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;

const FINGERPRINT_SHOWN: usize = 8;

/// The plan for a person: the date, the registry's day and the workspace,
/// one row per unit, the crates the units share, then every diagnostic in
/// text.
pub struct TableRenderer;

impl TableRenderer {
    fn diagnostics(diagnostics: &[Diagnostic]) -> String {
        diagnostics
            .iter()
            .flat_map(|diagnostic| [DiagnosticLine.line(diagnostic), String::from("\n")])
            .collect()
    }
}

impl DocumentRenderer for TableRenderer {
    fn check(&self, report: &CheckReport) -> String {
        CheckTable.text(report)
    }

    fn failure(&self, failure: &PlanFailure) -> String {
        Self::diagnostics(&failure.diagnostics)
    }

    fn guide(&self, text: &GuideText) -> String {
        GuideTable.text(text)
    }

    fn hold(&self, report: &HoldReport) -> String {
        HoldTable.text(report)
    }

    fn plan(&self, plan: &Plan) -> String {
        let fingerprint = plan
            .workspace
            .fingerprint
            .chars()
            .take(FINGERPRINT_SHOWN)
            .collect::<String>();
        let mut rows = vec![["unit", "kind", "from", "to", "outcome", "reasons"].map(String::from)];
        rows.extend(plan.units.iter().map(|planned| {
            let from = TargetCell.origin(&planned.unit);
            let decision = &planned.decision;
            let urgency = if decision.urgent { ", urgent" } else { "" };
            [
                RowLabel.unit(planned),
                String::from(planned.unit.kind.label()),
                from,
                TargetCell.text(&planned.unit),
                format!("{}{urgency}", decision.outcome.label()),
                decision.reasons.join("; "),
            ]
        }));
        let mut text = format!(
            "date {}, registry read {}, workspace {fingerprint}\n",
            plan.date, plan.registry_read
        );
        for line in Columns.lines(&rows) {
            text.push_str(line.trim_end());
            text.push('\n');
        }
        if !plan.shared.is_empty() {
            text.push_str("\nshared by the units: ");
            text.push_str(&plan.shared.join(", "));
            text.push_str(" -- their moves depend on the order the units are applied in\n");
        }
        if !plan.diagnostics.is_empty() {
            text.push('\n');
            text.push_str(&Self::diagnostics(&plan.diagnostics));
        }
        text
    }
}

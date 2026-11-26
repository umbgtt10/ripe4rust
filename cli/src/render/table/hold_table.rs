// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::table::diagnostic_line::DiagnosticLine;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;

/// The pins `hold` prints, for a person: where they go, the section's header
/// first where it is to be added, each pin's line, and what to do, then the
/// diagnostics.
pub struct HoldTable;

impl HoldTable {
    pub fn text(&self, report: &HoldReport) -> String {
        let mut lines = vec![match &report.at {
            Some(at) => format!("hold {}: under {} at {at}", report.unit, report.section),
            None => format!(
                "hold {}: in a new {} at the end of Cargo.toml",
                report.unit, report.section
            ),
        }];
        if report.at.is_none() {
            lines.push(report.section.clone());
        }
        lines.extend(report.pins.iter().map(|pin| pin.line.clone()));
        lines.push(format!("Do: {}", report.action));
        if !report.diagnostics.is_empty() {
            lines.push(String::new());
        }
        lines.extend(
            report
                .diagnostics
                .iter()
                .map(|diagnostic| DiagnosticLine.line(diagnostic)),
        );
        let mut text = lines.join("\n");
        text.push('\n');
        text
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::table::diagnostic_line::DiagnosticLine;
use crate::render::table::sentence::Sentence;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

const NONE: &str = "none";

/// What `check` found, for a person: whether the unit is as planned, the
/// moves a breaking unit's edit made, every difference with what to do, the
/// moves of crates the units share and the dependencies re-pointed between
/// versions the lock already held, each reported and never a difference, the
/// units applied and those still to go, then the diagnostics.
pub struct CheckTable;

impl CheckTable {
    pub fn text(&self, report: &CheckReport) -> String {
        let stands = if report.as_planned {
            "as planned"
        } else {
            "not as planned"
        };
        let mut lines = vec![format!("check {}: {stands}", report.unit)];
        if !report.transitive.is_empty() {
            let moves = report
                .transitive
                .iter()
                .map(Self::moved)
                .collect::<Vec<String>>();
            lines.push(format!("transitive: {}", moves.join(", ")));
        }
        lines.extend(report.differences.iter().map(|difference| {
            format!(
                "{}: {} Do: {}",
                difference.at,
                Sentence.ended(&difference.what),
                difference.action
            )
        }));
        lines.extend(
            report
                .shared
                .iter()
                .map(|what| format!("shared: {}", Sentence.ended(what))),
        );
        lines.extend(
            report
                .repointed
                .iter()
                .map(|what| format!("repointed: {}", Sentence.ended(what))),
        );
        for (label, ids) in [("applied", &report.applied), ("to go", &report.to_go)] {
            let listed = if ids.is_empty() {
                String::from(NONE)
            } else {
                ids.join(", ")
            };
            lines.push(format!("{label}: {listed}"));
        }
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

    fn moved(lock_move: &LockMove) -> String {
        let name = &lock_move.name;
        match (&lock_move.from, &lock_move.to) {
            (Some(from), Some(to)) => format!("{name} {from} -> {to}"),
            (None, Some(to)) => format!("{name} {to} added"),
            (from, None) => format!(
                "{name} {} removed",
                from.as_ref().map(ToString::to_string).unwrap_or_default()
            ),
        }
    }
}

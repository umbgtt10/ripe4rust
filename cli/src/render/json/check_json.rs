// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::diagnostic_json::DiagnosticJson;
use crate::render::json::documents::check_document::CheckDocument;
use crate::render::json::documents::difference_document::DifferenceDocument;
use crate::render::json::move_json::MoveJson;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

/// What `check` found, as the JSON document carries it.
pub struct CheckJson;

impl CheckJson {
    pub fn document<'a>(&self, report: &'a CheckReport) -> CheckDocument<'a> {
        CheckDocument {
            unit: &report.unit,
            as_planned: report.as_planned,
            transitive: report
                .transitive
                .iter()
                .map(|lock_move| MoveJson.document(lock_move))
                .collect(),
            differences: report
                .differences
                .iter()
                .map(|difference| DifferenceDocument {
                    at: &difference.at,
                    what: &difference.what,
                    action: &difference.action,
                })
                .collect(),
            shared: &report.shared,
            applied: &report.applied,
            to_go: &report.to_go,
            diagnostics: report
                .diagnostics
                .iter()
                .map(|diagnostic| DiagnosticJson.document(diagnostic))
                .collect(),
        }
    }
}

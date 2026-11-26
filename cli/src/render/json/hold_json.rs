// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::diagnostic_json::DiagnosticJson;
use crate::render::json::documents::hold_document::HoldDocument;
use crate::render::json::documents::pin_document::PinDocument;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;

/// The pins `hold` prints, as the JSON document carries them.
pub struct HoldJson;

impl HoldJson {
    pub fn document<'a>(&self, report: &'a HoldReport) -> HoldDocument<'a> {
        HoldDocument {
            unit: &report.unit,
            section: &report.section,
            at: report.at.as_ref().map(ToString::to_string),
            pins: report
                .pins
                .iter()
                .map(|pin| PinDocument {
                    name: &pin.name,
                    line: &pin.line,
                    replaces: pin.replaces.as_ref().map(ToString::to_string),
                })
                .collect(),
            action: &report.action,
            diagnostics: report
                .diagnostics
                .iter()
                .map(|diagnostic| DiagnosticJson.document(diagnostic))
                .collect(),
        }
    }
}

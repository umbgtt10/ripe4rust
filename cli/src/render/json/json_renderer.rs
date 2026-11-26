// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::document_renderer::DocumentRenderer;
use crate::render::json::check_json::CheckJson;
use crate::render::json::diagnostic_json::DiagnosticJson;
use crate::render::json::documents::diagnostic_document::DiagnosticDocument;
use crate::render::json::documents::unit_document::UnitDocument;
use crate::render::json::hold_json::HoldJson;
use crate::render::json::unit_json::UnitJson;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;
use serde::Serialize;
use serde_json::to_string_pretty;

const SCHEMA: u32 = 1;

#[derive(Serialize)]
struct Enveloped<'a, T: Serialize> {
    pub schema: u32,
    pub command: &'a str,
    #[serde(flatten)]
    pub body: T,
}

#[derive(Serialize)]
struct FailureDocument<'a> {
    pub schema: u32,
    pub command: &'a str,
    pub diagnostics: Vec<DiagnosticDocument<'a>>,
}

#[derive(Serialize)]
struct WrittenLineDocument<'a> {
    #[serde(rename = "where")]
    pub at: String,
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Serialize)]
struct WorkspaceDocument<'a> {
    pub fingerprint: &'a str,
    pub lock: &'a [String],
    pub lines: Vec<WrittenLineDocument<'a>>,
}

#[derive(Serialize)]
struct PlanDocument<'a> {
    pub schema: u32,
    pub command: &'a str,
    pub date: String,
    pub registry_read: String,
    pub workspace: WorkspaceDocument<'a>,
    pub gates: &'a [String],
    pub diagnostics: Vec<DiagnosticDocument<'a>>,
    pub units: Vec<UnitDocument<'a>>,
}

/// The one document a command prints with `--format json`, on success and
/// on failure alike: `schema`, which moves when the shape does, the command,
/// and the diagnostics -- and for a plan, a check or a hold, everything it
/// holds. Every object
/// is a record of its own, so its keys keep the design's order.
pub struct JsonRenderer {
    command: String,
}

impl JsonRenderer {
    pub fn new(command: &str) -> Self {
        Self {
            command: String::from(command),
        }
    }

    fn diagnostics(diagnostics: &[Diagnostic]) -> Vec<DiagnosticDocument<'_>> {
        diagnostics
            .iter()
            .map(|diagnostic| DiagnosticJson.document(diagnostic))
            .collect()
    }

    fn enveloped(&self, body: impl Serialize) -> String {
        Self::text(&Enveloped {
            schema: SCHEMA,
            command: &self.command,
            body,
        })
    }

    fn text(document: &impl Serialize) -> String {
        let mut text = to_string_pretty(document).unwrap_or_default();
        text.push('\n');
        text
    }
}

impl DocumentRenderer for JsonRenderer {
    fn check(&self, report: &CheckReport) -> String {
        self.enveloped(CheckJson.document(report))
    }

    fn failure(&self, failure: &PlanFailure) -> String {
        Self::text(&FailureDocument {
            schema: SCHEMA,
            command: &self.command,
            diagnostics: Self::diagnostics(&failure.diagnostics),
        })
    }

    fn hold(&self, report: &HoldReport) -> String {
        self.enveloped(HoldJson.document(report))
    }

    fn plan(&self, plan: &Plan) -> String {
        Self::text(&PlanDocument {
            schema: SCHEMA,
            command: &self.command,
            date: plan.date.to_string(),
            registry_read: plan.registry_read.to_string(),
            workspace: WorkspaceDocument {
                fingerprint: &plan.workspace.fingerprint,
                lock: &plan.workspace.lock,
                lines: plan
                    .workspace
                    .lines
                    .iter()
                    .map(|line| WrittenLineDocument {
                        at: line.at.to_string(),
                        key: &line.key,
                        value: &line.value,
                    })
                    .collect(),
            },
            gates: &plan.gates,
            diagnostics: Self::diagnostics(&plan.diagnostics),
            units: plan
                .units
                .iter()
                .map(|planned| UnitJson.document(planned))
                .collect(),
        })
    }
}

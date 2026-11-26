// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::diagnostic_json::DiagnosticJson;
use crate::render::json::unit_json::UnitJson;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use serde_json::to_string_pretty;

const SCHEMA: u32 = 1;

#[derive(Serialize)]
struct FailureDocument<'a> {
    pub schema: u32,
    pub command: &'a str,
    pub diagnostics: Vec<Value>,
}

#[derive(Serialize)]
struct PlanDocument<'a> {
    pub schema: u32,
    pub command: &'a str,
    pub date: String,
    pub registry_read: String,
    pub workspace: Value,
    pub gates: &'a [String],
    pub diagnostics: Vec<Value>,
    pub units: Vec<Value>,
}

/// The one document a command prints with `--format json`, on success and
/// on failure alike: `schema`, which moves when the shape does, the command,
/// and the diagnostics -- and for a plan, everything it holds. The
/// document's own keys keep the design's order; the objects within it are
/// written with their keys sorted.
pub struct JsonRenderer {
    command: String,
}

impl JsonRenderer {
    pub fn new(command: &str) -> Self {
        Self {
            command: String::from(command),
        }
    }

    pub fn failure(&self, failure: &PlanFailure) -> String {
        Self::text(&FailureDocument {
            schema: SCHEMA,
            command: &self.command,
            diagnostics: Self::diagnostics(&failure.diagnostics),
        })
    }

    pub fn plan(&self, plan: &Plan) -> String {
        Self::text(&PlanDocument {
            schema: SCHEMA,
            command: &self.command,
            date: plan.date.to_string(),
            registry_read: plan.registry_read.to_string(),
            workspace: json!({
                "fingerprint": plan.workspace.fingerprint,
                "lock": plan.workspace.lock,
            }),
            gates: &plan.gates,
            diagnostics: Self::diagnostics(&plan.diagnostics),
            units: plan
                .units
                .iter()
                .map(|planned| UnitJson.value(planned))
                .collect(),
        })
    }

    fn diagnostics(diagnostics: &[Diagnostic]) -> Vec<Value> {
        diagnostics
            .iter()
            .map(|diagnostic| DiagnosticJson.value(diagnostic))
            .collect()
    }

    fn text(document: &impl Serialize) -> String {
        let mut text = to_string_pretty(document).unwrap_or_default();
        text.push('\n');
        text
    }
}

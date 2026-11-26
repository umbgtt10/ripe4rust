// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use serde_json::Value;
use serde_json::json;

/// A diagnostic as the JSON document carries it.
pub struct DiagnosticJson;

impl DiagnosticJson {
    pub fn value(&self, diagnostic: &Diagnostic) -> Value {
        json!({
            "severity": diagnostic.severity.label(),
            "code": diagnostic.code.label(),
            "where": diagnostic.at.as_ref().map(ToString::to_string),
            "what": diagnostic.what,
            "do": diagnostic.action,
            "needs_person": diagnostic.needs_person,
        })
    }
}

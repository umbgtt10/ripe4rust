// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::diagnostic_document::DiagnosticDocument;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;

/// A diagnostic as the JSON document carries it.
pub struct DiagnosticJson;

impl DiagnosticJson {
    pub fn document<'a>(&self, diagnostic: &'a Diagnostic) -> DiagnosticDocument<'a> {
        DiagnosticDocument {
            severity: diagnostic.severity.label(),
            code: diagnostic.code.label(),
            at: diagnostic.at.as_ref().map(ToString::to_string),
            what: &diagnostic.what,
            action: &diagnostic.action,
            needs_person: diagnostic.needs_person,
        }
    }
}

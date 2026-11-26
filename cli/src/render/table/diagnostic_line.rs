// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic::Diagnostic;

/// A diagnostic in text: `error[<code>] <where>: <what>. Do: <do>`.
pub struct DiagnosticLine;

impl DiagnosticLine {
    pub fn line(&self, diagnostic: &Diagnostic) -> String {
        let place = diagnostic
            .at
            .as_ref()
            .map(|at| format!("{at}: "))
            .unwrap_or_default();
        format!(
            "{}[{}] {place}{}. Do: {}",
            diagnostic.severity.label(),
            diagnostic.code.label(),
            diagnostic.what,
            diagnostic.action
        )
    }
}

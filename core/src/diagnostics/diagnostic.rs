// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::diagnostics::severity::Severity;
use crate::values::location::Location;
use alloc::string::String;

/// A problem found on the way: where it is, what happened, and the next step,
/// written for an agent with no context (`action` is the document's `do`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub at: Option<Location>,
    pub what: String,
    pub action: String,
    pub needs_person: bool,
}

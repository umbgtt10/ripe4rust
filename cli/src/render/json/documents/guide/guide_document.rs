// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::diagnostic_document::DiagnosticDocument;
use crate::render::json::documents::guide::code_entry_document::CodeEntryDocument;
use crate::render::json::documents::guide::command_entry_document::CommandEntryDocument;
use crate::render::json::documents::guide::field_entry_document::FieldEntryDocument;
use serde::Serialize;

/// The guide, after the document's `schema` and `command`, in the design's
/// order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GuideDocument<'a> {
    pub version: &'a str,
    pub round: &'a [String],
    pub commands: Vec<CommandEntryDocument<'a>>,
    pub codes: Vec<CodeEntryDocument<'a>>,
    pub fields: Vec<FieldEntryDocument<'a>>,
    pub diagnostics: Vec<DiagnosticDocument<'a>>,
}

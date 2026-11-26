// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::diagnostic_document::DiagnosticDocument;
use crate::render::json::documents::difference_document::DifferenceDocument;
use crate::render::json::documents::move_document::MoveDocument;
use serde::Serialize;

/// What `check` found, after the document's `schema` and `command`, in
/// the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CheckDocument<'a> {
    pub unit: &'a str,
    pub as_planned: bool,
    pub transitive: Vec<MoveDocument<'a>>,
    pub differences: Vec<DifferenceDocument<'a>>,
    pub applied: &'a [String],
    pub to_go: &'a [String],
    pub diagnostics: Vec<DiagnosticDocument<'a>>,
}

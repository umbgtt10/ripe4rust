// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::diagnostic_document::DiagnosticDocument;
use crate::render::json::documents::pin_document::PinDocument;
use serde::Serialize;

/// The pins `hold` prints, after the document's `schema` and `command`, in
/// the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct HoldDocument<'a> {
    pub unit: &'a str,
    pub section: &'a str,
    #[serde(rename = "where")]
    pub at: Option<String>,
    pub pins: Vec<PinDocument<'a>>,
    #[serde(rename = "do")]
    pub action: &'a str,
    pub diagnostics: Vec<DiagnosticDocument<'a>>,
}

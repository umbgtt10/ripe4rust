// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// A diagnostic as the JSON document writes it, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DiagnosticDocument<'a> {
    pub severity: &'a str,
    pub code: &'a str,
    #[serde(rename = "where")]
    pub at: Option<String>,
    pub what: &'a str,
    #[serde(rename = "do")]
    pub action: &'a str,
    pub needs_person: bool,
}

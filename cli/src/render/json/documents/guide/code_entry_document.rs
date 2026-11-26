// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// A diagnostic code as the catalogue gives it, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CodeEntryDocument<'a> {
    pub code: &'a str,
    pub severity: &'a str,
    pub needs_person: bool,
    pub means: &'a str,
    #[serde(rename = "do")]
    pub remedy: &'a str,
}

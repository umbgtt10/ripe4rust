// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// A dependent holding a breaking unit's crate back, as the document
/// carries it: the package, its version, the crate it requires and how.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DependentDocument<'a> {
    #[serde(rename = "crate")]
    pub name: &'a str,
    pub version: String,
    pub requires: &'a str,
    pub requirement: String,
}

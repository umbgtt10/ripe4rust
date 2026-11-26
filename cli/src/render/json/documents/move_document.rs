// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// A lock move: the crate, and the versions it leaves and takes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MoveDocument<'a> {
    #[serde(rename = "crate")]
    pub name: &'a str,
    pub from: Option<String>,
    pub to: Option<String>,
}

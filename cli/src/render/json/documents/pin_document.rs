// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// One pin `hold` prints, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PinDocument<'a> {
    #[serde(rename = "crate")]
    pub name: &'a str,
    pub line: &'a str,
    pub replaces: Option<String>,
}

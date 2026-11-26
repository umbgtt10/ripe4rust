// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// One way a unit is not as planned, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DifferenceDocument<'a> {
    #[serde(rename = "where")]
    pub at: &'a str,
    pub what: &'a str,
    #[serde(rename = "do")]
    pub action: &'a str,
}

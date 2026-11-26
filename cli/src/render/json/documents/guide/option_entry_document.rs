// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// What a command takes, and what it means, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OptionEntryDocument<'a> {
    pub option: &'a str,
    pub means: &'a str,
}

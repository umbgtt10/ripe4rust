// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// One way a unit is not as planned: where -- `Cargo.lock`, or a manifest's
/// `file:line` --, what, and what to do (`action` is the document's `do`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    pub at: String,
    pub what: String,
    pub action: String,
}

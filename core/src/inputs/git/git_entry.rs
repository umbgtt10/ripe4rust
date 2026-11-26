// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A crate of a git source the lock holds: its name, its source -- the URL
/// and its spec --, and the short rev the lock holds it at.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitEntry {
    pub name: String,
    pub source: String,
    pub rev: String,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;

/// A crates.io package the lock holds: its name and its version.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockEntry {
    pub name: String,
    pub version: Version,
}

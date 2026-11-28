// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// What the parse found of one crate: every line naming it, and the
/// workspace's packages whose public API its items reach.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateUse {
    pub name: String,
    pub locations: Vec<Location>,
    pub exposed_in: Vec<String>,
}

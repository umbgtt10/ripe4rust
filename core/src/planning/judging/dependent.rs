// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::requirement::Requirement;
use crate::values::version::Version;
use alloc::string::String;

/// A package of the lock whose requirement keeps a crate a breaking unit
/// moves on the version in use: the package, the crate it requires and the
/// requirement, which the unit's target does not meet, and the versions the
/// unit leaves and takes the crate to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependent {
    pub name: String,
    pub version: Version,
    pub requires: String,
    pub requirement: Requirement,
    pub in_use: Version,
    pub target: Version,
}

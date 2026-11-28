// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;

/// A dependency the lock writes with its version, as it does where it holds
/// the crate at more than one: the package that depends -- a member, a
/// crates.io package or a git source's crate -- by name and version, and the
/// crates.io package it depends on, by name and version.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockEdge {
    pub dependent: String,
    pub dependent_version: Version,
    pub dependency: String,
    pub dependency_version: Version,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::resolved_dependency::ResolvedDependency;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// A registry package in the lock, with what it depends on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockedPackage {
    pub name: String,
    pub version: Version,
    pub dependencies: Vec<ResolvedDependency>,
}

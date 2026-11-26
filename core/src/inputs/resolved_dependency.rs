// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::requirement::Requirement;
use crate::values::version::Version;
use alloc::string::String;

/// A dependency as the lock resolved it: the version taken and the
/// requirement that asked for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: Version,
    pub requirement: Requirement,
}

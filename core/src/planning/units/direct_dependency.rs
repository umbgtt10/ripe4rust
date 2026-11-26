// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// A registry crate the workspace's members depend on, the version in use
/// and the members that use it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectDependency {
    pub name: String,
    pub version: Version,
    pub members: Vec<String>,
}

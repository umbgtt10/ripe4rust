// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::resolved_dependency::ResolvedDependency;
use crate::values::partial_version::PartialVersion;
use alloc::string::String;
use alloc::vec::Vec;

/// A package of the workspace, its declared `rust-version` and its
/// dependencies of every kind, normal, build and dev.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Member {
    pub name: String,
    pub rust_version: Option<PartialVersion>,
    pub dependencies: Vec<ResolvedDependency>,
}

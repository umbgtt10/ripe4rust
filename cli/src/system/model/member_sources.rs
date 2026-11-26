// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::declared_dependency::DeclaredDependency;
use crate::system::model::source_target::SourceTarget;

/// What the parse reads of a member: its targets, and the dependencies its
/// code can name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberSources {
    pub name: String,
    pub targets: Vec<SourceTarget>,
    pub dependencies: Vec<DeclaredDependency>,
}

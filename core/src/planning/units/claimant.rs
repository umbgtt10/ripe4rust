// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::dependency_moves::DependencyMoves;
use crate::planning::units::source_moves::SourceMoves;

/// What claims the lock's moves, in plan order: a direct dependency, or a
/// git source moving.
pub enum Claimant {
    Dependency(DependencyMoves),
    Source(SourceMoves),
}

impl Claimant {
    pub fn moves(&self) -> &[LockMove] {
        match self {
            Self::Dependency(dependency) => &dependency.moves,
            Self::Source(source) => &source.moves,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Dependency(dependency) => &dependency.dependency.name,
            Self::Source(source) => &source.name,
        }
    }
}

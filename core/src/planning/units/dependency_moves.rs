// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::direct_dependency::DirectDependency;
use alloc::vec::Vec;

/// What `cargo update --dry-run --recursive -p <crate>` moves for one direct
/// dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyMoves {
    pub dependency: DirectDependency,
    pub moves: Vec<LockMove>,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_move::GitMove;
use crate::inputs::lock_move::LockMove;
use alloc::vec::Vec;

/// What `cargo update --dry-run` would move: the registry's crates, and the
/// crates of git sources apart.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Moves {
    pub registry: Vec<LockMove>,
    pub git: Vec<GitMove>,
}

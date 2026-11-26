// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::moves::Moves;
use crate::inputs::lock_move::LockMove;
use crate::inputs::port_failure::PortFailure;
use crate::values::version::Version;
use alloc::vec::Vec;

/// What `cargo update --dry-run` would move: for one direct dependency, with
/// `--recursive -p`; for a git source, by one of its crates with `-p`; and
/// for the whole workspace, git sources apart.
pub trait LockMoves {
    fn of_crate(&self, name: &str, version: &Version) -> Result<Vec<LockMove>, PortFailure>;
    fn of_source(&self, name: &str, version: &Version) -> Result<Moves, PortFailure>;
    fn of_workspace(&self) -> Result<Moves, PortFailure>;
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_source::GitSource;
use crate::inputs::lock_move::LockMove;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// What `cargo update --dry-run -p <crate>` moves for a git source moving:
/// the source and its revs, the crate in the lock it is asked by at its
/// version, every crate of it that moves, and the registry's moves it pulls
/// in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceMoves {
    pub source: GitSource,
    pub name: String,
    pub version: Version,
    pub crates: Vec<String>,
    pub moves: Vec<LockMove>,
}

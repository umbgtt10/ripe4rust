// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_entry::GitEntry;
use crate::inputs::lock_entry::LockEntry;
use crate::inputs::written_line::WrittenLine;
use alloc::vec::Vec;

/// The workspace as the round finds it now: the lock's crates.io packages,
/// every line a unit can edit, and the crates of git sources at their revs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentState {
    pub lock: Vec<LockEntry>,
    pub lines: Vec<WrittenLine>,
    pub git: Vec<GitEntry>,
}

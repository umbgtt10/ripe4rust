// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_entry::LockEntry;
use crate::inputs::written_line::WrittenLine;
use alloc::vec::Vec;

/// The workspace as the round finds it now: the lock's crates.io packages
/// and every line a unit can edit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CurrentState {
    pub lock: Vec<LockEntry>,
    pub lines: Vec<WrittenLine>,
}

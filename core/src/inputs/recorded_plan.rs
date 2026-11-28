// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_entry::GitEntry;
use crate::inputs::lock_edge::LockEdge;
use crate::inputs::lock_entry::LockEntry;
use crate::inputs::recorded_unit::RecordedUnit;
use crate::inputs::written_line::WrittenLine;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec::Vec;

/// A plan as `plan` wrote it, what `check` and `hold` read of it: its date,
/// the workspace it was made from -- the fingerprint, the lock, the lines a
/// unit can edit, the crates of git sources at their revs, the lock's
/// versioned edges, `None` for a plan written before they were recorded --,
/// the crates its units share, and its units, in plan order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedPlan {
    pub date: Date,
    pub fingerprint: String,
    pub lock: Vec<LockEntry>,
    pub lines: Vec<WrittenLine>,
    pub git: Vec<GitEntry>,
    pub edges: Option<Vec<LockEdge>>,
    pub shared: Vec<String>,
    pub units: Vec<RecordedUnit>,
}

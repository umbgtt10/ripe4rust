// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_entry::LockEntry;
use crate::inputs::recorded_unit::RecordedUnit;
use crate::inputs::written_line::WrittenLine;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec::Vec;

/// A plan as `plan` wrote it, what `check` and `hold` read of it: its date,
/// the workspace it was made from -- the fingerprint, the lock, the lines a
/// unit can edit -- and its units, in plan order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedPlan {
    pub date: Date,
    pub fingerprint: String,
    pub lock: Vec<LockEntry>,
    pub lines: Vec<WrittenLine>,
    pub units: Vec<RecordedUnit>,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::recorded_unit::RecordedUnit;
use crate::planning::round::git_changes::GitChanges;
use crate::planning::round::line_changes::LineChanges;
use crate::planning::round::lock_changes::LockChanges;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// How far a unit is applied: wholly, every move in the lock, every edit
/// written and a git unit's source at the rev it takes; not at all; or partly, listed by the ids of its split where it
/// has one, and as still to go where it has none.
pub struct Progress<'a> {
    lock: &'a LockChanges,
    lines: &'a LineChanges,
    git: &'a GitChanges,
}

impl<'a> Progress<'a> {
    pub fn new(lock: &'a LockChanges, lines: &'a LineChanges, git: &'a GitChanges) -> Self {
        Self { lock, lines, git }
    }

    pub fn applied(&self, unit: &RecordedUnit) -> Vec<String> {
        if self.whole(unit) {
            vec![unit.id.clone()]
        } else {
            self.splits(unit, true)
        }
    }

    pub fn to_go(&self, unit: &RecordedUnit) -> Vec<String> {
        if self.whole(unit) {
            Vec::new()
        } else if unit.splits.is_empty() || !self.parts(unit).contains(&true) {
            vec![unit.id.clone()]
        } else {
            self.splits(unit, false)
        }
    }

    pub fn whole(&self, unit: &RecordedUnit) -> bool {
        let parts = self.parts(unit);
        !parts.is_empty() && !parts.contains(&false)
    }

    fn parts(&self, unit: &RecordedUnit) -> Vec<bool> {
        unit.moves
            .iter()
            .map(|lock_move| self.lock.applied(lock_move))
            .chain(unit.edits.iter().map(|edit| self.lines.made(edit)))
            .chain(unit.git.iter().map(|source| self.git.applied(source)))
            .collect()
    }

    fn splits(&self, unit: &RecordedUnit, applied: bool) -> Vec<String> {
        let prefix = format!("{}/", unit.id);
        unit.splits
            .iter()
            .filter(|id| {
                let name = id.strip_prefix(&prefix).unwrap_or_default();
                unit.moves
                    .iter()
                    .find(|lock_move| lock_move.name == name)
                    .is_some_and(|lock_move| self.lock.applied(lock_move))
                    == applied
            })
            .cloned()
            .collect()
    }
}

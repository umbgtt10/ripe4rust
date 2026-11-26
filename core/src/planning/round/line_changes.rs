// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::written_line::WrittenLine;
use crate::planning::round::line_change::LineChange;
use crate::values::location::Location;
use alloc::vec::Vec;

/// The lines a unit can edit, as the plan found them and as they are now.
/// A line stands for the one the plan found where it is the same key, in the
/// same file, in the same place among that key's lines there: a line moved
/// down the file by an edit above it is still the same line.
pub struct LineChanges {
    base: Vec<WrittenLine>,
    current: Vec<WrittenLine>,
}

impl LineChanges {
    pub fn new(base: &[WrittenLine], current: &[WrittenLine]) -> Self {
        Self {
            base: base.to_vec(),
            current: current.to_vec(),
        }
    }

    pub fn changes(&self) -> Vec<LineChange> {
        let rewritten = self.base.iter().filter_map(|then| {
            match Self::counterpart(&self.base, &self.current, then) {
                None => Some(LineChange::Gone { then: then.clone() }),
                Some(now) if now.value != then.value => Some(LineChange::Rewritten {
                    then: then.clone(),
                    now: now.clone(),
                }),
                Some(_) => None,
            }
        });
        let new = self
            .current
            .iter()
            .filter(|now| Self::counterpart(&self.current, &self.base, now).is_none())
            .map(|now| LineChange::New { now: now.clone() });
        rewritten.chain(new).collect()
    }

    pub fn made(&self, edit: &WrittenLine) -> bool {
        self.now(&edit.at, &edit.key)
            .is_some_and(|now| now.value == edit.value)
    }

    pub fn now(&self, at: &Location, key: &str) -> Option<WrittenLine> {
        self.base
            .iter()
            .find(|then| then.at == *at && then.key == key)
            .and_then(|then| Self::counterpart(&self.base, &self.current, then))
            .cloned()
    }

    fn counterpart<'l>(
        from: &[WrittenLine],
        into: &'l [WrittenLine],
        line: &WrittenLine,
    ) -> Option<&'l WrittenLine> {
        let alike = |other: &&WrittenLine| other.at.file == line.at.file && other.key == line.key;
        let place = from
            .iter()
            .filter(alike)
            .filter(|other| other.at.line < line.at.line)
            .count();
        let mut matching = into.iter().filter(alike).collect::<Vec<&WrittenLine>>();
        matching.sort_by_key(|other| other.at.line);
        matching.get(place).copied()
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_entry::LockEntry;
use crate::inputs::lock_move::LockMove;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// The lock as the plan found it and as it is now: whether a move shows in
/// it -- its old version gone, its new one there --, whether a version is
/// held by both, and the changes no move accounts for, paired crate by crate.
pub struct LockChanges {
    base: Vec<LockEntry>,
    current: Vec<LockEntry>,
}

impl LockChanges {
    pub fn new(base: &[LockEntry], current: &[LockEntry]) -> Self {
        Self {
            base: base.to_vec(),
            current: current.to_vec(),
        }
    }

    pub fn applied(&self, lock_move: &LockMove) -> bool {
        let name = &lock_move.name;
        let left = lock_move
            .from
            .as_ref()
            .is_none_or(|from| !self.holds(name, from));
        let arrived = lock_move.to.as_ref().is_none_or(|to| self.holds(name, to));
        let changed = lock_move.from.is_some()
            || lock_move
                .to
                .as_ref()
                .is_some_and(|to| !Self::within(&self.base, name, to));
        left && arrived && changed
    }

    pub fn holds(&self, name: &str, version: &Version) -> bool {
        Self::within(&self.current, name, version)
    }

    pub fn kept(&self, name: &str, version: &Version) -> bool {
        Self::within(&self.base, name, version) && self.holds(name, version)
    }

    pub fn unexplained(&self, applied: &[LockMove]) -> Vec<LockMove> {
        let left = self
            .base
            .iter()
            .filter(|entry| !Self::within(&self.current, &entry.name, &entry.version))
            .filter(|entry| {
                !applied.iter().any(|lock_move| {
                    lock_move.name == entry.name && lock_move.from.as_ref() == Some(&entry.version)
                })
            })
            .collect::<Vec<&LockEntry>>();
        let joined = self
            .current
            .iter()
            .filter(|entry| !Self::within(&self.base, &entry.name, &entry.version))
            .filter(|entry| {
                !applied.iter().any(|lock_move| {
                    lock_move.name == entry.name && lock_move.to.as_ref() == Some(&entry.version)
                })
            })
            .collect::<Vec<&LockEntry>>();
        let mut names = left
            .iter()
            .chain(&joined)
            .map(|entry| entry.name.clone())
            .collect::<Vec<String>>();
        names.sort();
        names.dedup();
        names
            .iter()
            .flat_map(|name| Self::paired(name, &left, &joined))
            .collect()
    }

    fn paired(name: &str, left: &[&LockEntry], joined: &[&LockEntry]) -> Vec<LockMove> {
        let versions = |entries: &[&LockEntry]| {
            let mut versions = entries
                .iter()
                .filter(|entry| entry.name == name)
                .map(|entry| entry.version.clone())
                .collect::<Vec<Version>>();
            versions.sort();
            versions
        };
        let (from, to) = (versions(left), versions(joined));
        (0..from.len().max(to.len()))
            .map(|index| LockMove {
                name: String::from(name),
                from: from.get(index).cloned(),
                to: to.get(index).cloned(),
            })
            .collect()
    }

    fn within(entries: &[LockEntry], name: &str, version: &Version) -> bool {
        entries
            .iter()
            .any(|entry| entry.name == name && entry.version == *version)
    }
}

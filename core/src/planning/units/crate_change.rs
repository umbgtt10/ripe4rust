// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::values::version::Version;
use alloc::vec::Vec;

/// What a set of lock moves does to one crate, its versions apart: those
/// it leaves and those it reaches, each in order. Moves compare as changes
/// whether cargo printed a version moving as one line or as a removal and
/// an addition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateChange {
    left: Vec<Version>,
    reached: Vec<Version>,
}

impl CrateChange {
    pub fn union(changes: &[Self]) -> Self {
        Self::new(
            changes
                .iter()
                .flat_map(|change| change.left.iter().cloned())
                .collect(),
            changes
                .iter()
                .flat_map(|change| change.reached.iter().cloned())
                .collect(),
        )
    }

    pub fn new(left: Vec<Version>, reached: Vec<Version>) -> Self {
        let sorted = |versions: Vec<Version>| {
            let mut versions = versions;
            versions.sort();
            versions.dedup();
            versions
        };
        Self {
            left: sorted(left),
            reached: sorted(reached),
        }
    }

    pub fn of(name: &str, moves: &[LockMove]) -> Self {
        let named = moves.iter().filter(|lock_move| lock_move.name == name);
        Self::new(
            named
                .clone()
                .filter_map(|lock_move| lock_move.from.clone())
                .collect(),
            named.filter_map(|lock_move| lock_move.to.clone()).collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.left.is_empty() && self.reached.is_empty()
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.left.iter().any(|version| other.left.contains(version))
    }

    pub fn within(&self, other: &Self) -> bool {
        self.left.iter().all(|version| other.left.contains(version))
            && self
                .reached
                .iter()
                .all(|version| other.reached.contains(version))
    }
}

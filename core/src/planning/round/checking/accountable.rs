// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::round::lock_changes::LockChanges;
use crate::planning::round::ownership::Ownership;
use crate::planning::round::target::Target;
use alloc::vec::Vec;

/// What the unit checked answers for: the crates it moves, the moves it
/// owns -- a split's move always, as its step takes it alone --, those of
/// them the lock does not show, and the crates whose changes are its to
/// explain: every crate the units do not share, and its own crate or its
/// split's though they do.
pub struct Accountable<'a> {
    target: &'a Target,
    ownership: Ownership<'a>,
    lock: &'a LockChanges,
}

impl<'a> Accountable<'a> {
    pub fn new(target: &'a Target, ownership: Ownership<'a>, lock: &'a LockChanges) -> Self {
        Self {
            target,
            ownership,
            lock,
        }
    }

    pub fn answers_for(&self, name: &str) -> bool {
        let own = self.target.unit.members.iter().any(|member| member == name)
            || (self.split() && self.target.moves.iter().any(|planned| planned.name == name));
        own || !self.ownership.is_shared(name)
    }

    pub fn moves(&self, name: &str) -> bool {
        self.target.moves.iter().any(|planned| planned.name == name)
    }

    pub fn owns(&self, lock_move: &LockMove) -> bool {
        self.split() || self.ownership.owns(&self.target.unit, lock_move)
    }

    pub fn unmade(&self) -> Vec<&'a LockMove> {
        self.target
            .moves
            .iter()
            .filter(|lock_move| !self.lock.applied(lock_move))
            .collect()
    }

    fn split(&self) -> bool {
        self.target.id != self.target.unit.id
    }
}

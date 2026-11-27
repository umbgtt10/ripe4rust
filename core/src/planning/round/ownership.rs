// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::inputs::recorded_unit::RecordedUnit;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec::Vec;

/// Which of a unit's moves the round holds it to: every move of a crate the
/// units do not share; and of a crate they share, whose moves depend on the
/// order they are applied in, the move of its own crate and the moves its
/// steps take one at a time -- the rest's, each one `--precise` update. A
/// unit that owns none of its moves is listed applied by them all.
pub struct Ownership<'a> {
    shared: &'a [String],
}

impl<'a> Ownership<'a> {
    pub fn new(shared: &'a [String]) -> Self {
        Self { shared }
    }

    pub fn is_shared(&self, name: &str) -> bool {
        self.shared.iter().any(|known| known == name)
    }

    pub fn owned(&self, unit: &RecordedUnit) -> Vec<LockMove> {
        let owned = unit
            .moves
            .iter()
            .filter(|lock_move| self.owns(unit, lock_move))
            .cloned()
            .collect::<Vec<LockMove>>();
        if owned.is_empty() {
            unit.moves.clone()
        } else {
            owned
        }
    }

    pub fn owns(&self, unit: &RecordedUnit, lock_move: &LockMove) -> bool {
        let rest = unit.kind == UnitKind::Compatible && unit.name.is_none();
        let stepped = rest && lock_move.from.is_some() && lock_move.to.is_some();
        !self.is_shared(&lock_move.name) || unit.members.contains(&lock_move.name) || stepped
    }
}

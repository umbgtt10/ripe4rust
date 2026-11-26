// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;

/// What a unit does with its own crate, the one it is named for: takes it to
/// a version, has its move held by a pin -- a member's move among those held
/// --, or, for a compatible unit, leaves it where it is and moves only what it
/// pulls in. Either of the last two takes it to no version.
pub struct OwnMove;

impl OwnMove {
    pub fn held(&self, unit: &Unit) -> bool {
        unit.held
            .iter()
            .any(|lock_move| unit.members.contains(&lock_move.name))
    }

    pub fn lands_on<'u>(&self, unit: &'u Unit) -> Option<&'u Version> {
        unit.to
            .as_ref()
            .or_else(|| unit.from.as_ref().filter(|_| self.stays(unit)))
    }

    pub fn stays(&self, unit: &Unit) -> bool {
        let Some(name) = &unit.name else {
            return false;
        };
        unit.kind == UnitKind::Compatible
            && !unit
                .moves
                .iter()
                .chain(&unit.held)
                .any(|lock_move| lock_move.name == *name)
    }
}

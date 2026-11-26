// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::vec::Vec;

/// The plan's order: units by crate name, a crate's compatible unit before
/// its breaking one, the toolchain as `rust`; then the rest; then the
/// advisory rows.
pub struct PlanOrder;

impl PlanOrder {
    pub fn sorted(&self, units: Vec<Unit>) -> Vec<Unit> {
        let mut units = units;
        units.sort_by(|left, right| {
            Self::place(left)
                .cmp(&Self::place(right))
                .then_with(|| left.name.cmp(&right.name))
                .then_with(|| Self::kind(left.kind).cmp(&Self::kind(right.kind)))
                .then_with(|| left.id.cmp(&right.id))
        });
        units
    }

    fn kind(kind: UnitKind) -> u8 {
        match kind {
            UnitKind::Compatible => 0,
            UnitKind::Breaking => 1,
            UnitKind::Toolchain => 2,
            UnitKind::Advisory => 3,
        }
    }

    fn place(unit: &Unit) -> u8 {
        match (unit.kind, &unit.name) {
            (UnitKind::Advisory, _) => 2,
            (_, None) => 1,
            _ => 0,
        }
    }
}

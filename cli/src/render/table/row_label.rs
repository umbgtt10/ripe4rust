// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::planned_unit::PlannedUnit;
use ripe4rust_core::unit_kind::UnitKind;

/// The unit column: the crate, a group's other members as `(+N)`, the moves
/// a compatible unit pulls in as `(+N transitive)`; the rest as `the rest`.
pub struct RowLabel;

impl RowLabel {
    pub fn unit(&self, planned: &PlannedUnit) -> String {
        let unit = &planned.unit;
        let Some(name) = &unit.name else {
            return String::from("the rest");
        };
        let others = unit.members.len().saturating_sub(1);
        let group = if others > 0 {
            format!(" (+{others})")
        } else {
            String::new()
        };
        let pulled = unit
            .moves
            .iter()
            .filter(|lock_move| !unit.members.contains(&lock_move.name))
            .count();
        let transitive = if unit.kind == UnitKind::Compatible && pulled > 0 {
            format!(" (+{pulled} transitive)")
        } else {
            String::new()
        };
        format!("{name}{group}{transitive}")
    }
}

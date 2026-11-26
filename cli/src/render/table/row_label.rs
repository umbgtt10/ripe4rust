// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::planned_unit::PlannedUnit;
use ripe4rust_core::unit_kind::UnitKind;

/// The unit column: the crate, then in one parenthesis a group's other
/// members as `+N released with it` -- a git source's other crates as `+N
/// from its source` -- and the moves a compatible or git unit pulls in as `+N
/// transitive`; the rest as `the rest`.
pub struct RowLabel;

impl RowLabel {
    pub fn unit(&self, planned: &PlannedUnit) -> String {
        let unit = &planned.unit;
        let Some(name) = &unit.name else {
            return String::from("the rest");
        };
        let others = unit.members.len().saturating_sub(1);
        let pulled = unit
            .moves
            .iter()
            .filter(|lock_move| !unit.members.contains(&lock_move.name))
            .count();
        let together = if unit.kind == UnitKind::Git {
            "from its source"
        } else {
            "released with it"
        };
        let counts = [
            (others > 0).then(|| format!("+{others} {together}")),
            (matches!(unit.kind, UnitKind::Compatible | UnitKind::Git) && pulled > 0)
                .then(|| format!("+{pulled} transitive")),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
        if counts.is_empty() {
            name.clone()
        } else {
            format!("{name} ({})", counts.join(", "))
        }
    }
}

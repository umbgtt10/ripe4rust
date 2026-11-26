// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::units::unit::Unit;

const HELD: &str = "held";
const NONE: &str = "--";

/// The `to` column: the version a unit takes its crate to, `held` where a
/// pin holds the crate's own move, and `--` where it takes none, as the rest.
pub struct TargetCell;

impl TargetCell {
    pub fn text(&self, unit: &Unit) -> String {
        let own_held = unit
            .held
            .iter()
            .any(|lock_move| unit.members.contains(&lock_move.name));
        match &unit.to {
            Some(to) => to.to_string(),
            None if own_held => String::from(HELD),
            None => String::from(NONE),
        }
    }
}

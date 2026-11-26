// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::units::own_move::OwnMove;
use ripe4rust_core::planning::units::unit::Unit;

const HELD: &str = "held";
const NONE: &str = "--";
const STAYS: &str = "stays";

/// The `from` column, the version a unit leaves or `--`; and the `to` column:
/// a git unit's revs in both. The version a unit takes its crate to, `held` where a
/// pin holds the crate's own move, `stays` where the crate does not move, only
/// what it pulls in, and `--` where it takes none, as the rest.
pub struct TargetCell;

impl TargetCell {
    pub fn origin(&self, unit: &Unit) -> String {
        match (&unit.git, &unit.from) {
            (Some(git), _) => git.from.clone(),
            (None, Some(from)) => from.to_string(),
            (None, None) => String::from(NONE),
        }
    }

    pub fn text(&self, unit: &Unit) -> String {
        if let Some(git) = &unit.git {
            return git.to.clone();
        }
        match &unit.to {
            Some(to) => to.to_string(),
            None if OwnMove.held(unit) => String::from(HELD),
            None if OwnMove.stays(unit) => String::from(STAYS),
            None => String::from(NONE),
        }
    }
}

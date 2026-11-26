// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::surveyed::Surveyed;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec::Vec;

/// The targets needing more than the gates a unit reaches: every one, for
/// the toolchain; for any other, those whose build reaches a crate it moves.
pub struct Reach<'a> {
    surveyed: &'a Surveyed,
}

impl<'a> Reach<'a> {
    pub fn new(surveyed: &'a Surveyed) -> Self {
        Self { surveyed }
    }

    pub fn targets(&self, unit: &Unit) -> Vec<String> {
        if unit.kind == UnitKind::Toolchain {
            return self.surveyed.policy.validate_beyond_gates.clone();
        }
        self.surveyed
            .reached
            .iter()
            .filter(|(_, crates)| {
                unit.members.iter().any(|member| crates.contains(member))
                    || unit
                        .moves
                        .iter()
                        .any(|lock_move| crates.contains(&lock_move.name))
            })
            .map(|(target, _)| target.clone())
            .collect()
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::units::concern::Concern;
use crate::planning::units::unit::Unit;
use alloc::vec;
use alloc::vec::Vec;

/// Every advisory and yanked release lands on the first unit, in plan
/// order, whose moves clear it, wherever it sits in the tree; one no unit
/// clears is unclear, and gets a row of its own.
pub struct Clearing {
    concerns: Vec<Concern>,
}

impl Clearing {
    pub fn new(concerns: Vec<Concern>) -> Self {
        Self { concerns }
    }

    pub fn assign(&self, units: &[Unit]) -> Vec<Vec<Concern>> {
        let mut assigned = vec![Vec::new(); units.len()];
        for concern in &self.concerns {
            if let Some(index) = units.iter().position(|unit| Self::clears(unit, concern)) {
                assigned[index].push(concern.clone());
            }
        }
        assigned
    }

    pub fn unclear(&self, units: &[Unit]) -> Vec<Concern> {
        self.concerns
            .iter()
            .filter(|concern| !units.iter().any(|unit| Self::clears(unit, concern)))
            .cloned()
            .collect()
    }

    fn clears(unit: &Unit, concern: &Concern) -> bool {
        unit.moves
            .iter()
            .any(|lock_move| concern.is_cleared_by(lock_move))
    }
}

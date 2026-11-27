// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::judging::facts::dependents::Dependents;
use crate::planning::units::concern::Concern;
use crate::planning::units::unit::Unit;
use alloc::vec;
use alloc::vec::Vec;

/// Every advisory and yanked release lands on the first unit, in plan
/// order, whose moves clear it, wherever it sits in the tree; one no unit
/// clears is unclear, and gets a row: one per crate version, holding every
/// advisory against it and its yanking. A move a dependent's requirement
/// holds back clears nothing, as the version it leaves stays for that
/// dependent: the row holds it out, as the move that would clear it.
pub struct Clearing<'a> {
    concerns: Vec<Concern>,
    dependents: Dependents<'a>,
}

impl<'a> Clearing<'a> {
    pub fn new(concerns: Vec<Concern>, dependents: Dependents<'a>) -> Self {
        Self {
            concerns,
            dependents,
        }
    }

    pub fn assign(&self, units: &[Unit]) -> Vec<Vec<Concern>> {
        let mut assigned = vec![Vec::new(); units.len()];
        for concern in &self.concerns {
            if let Some(index) = units.iter().position(|unit| self.clears(unit, concern)) {
                assigned[index].push(concern.clone());
            }
        }
        assigned
    }

    pub fn rows(&self, units: &[Unit]) -> Vec<(Unit, Vec<Concern>)> {
        let mut rows: Vec<(Unit, Vec<Concern>)> = Vec::new();
        for concern in self.unclear(units) {
            let mut row = concern.row();
            row.held = self.held_back(units, &concern);
            match rows.iter_mut().find(|(known, _)| known.id == row.id) {
                Some((known, concerns)) => {
                    known.held.extend(row.held);
                    known.held.dedup();
                    concerns.push(concern);
                }
                None => rows.push((row, vec![concern])),
            }
        }
        rows
    }

    pub fn unclear(&self, units: &[Unit]) -> Vec<Concern> {
        self.concerns
            .iter()
            .filter(|concern| !units.iter().any(|unit| self.clears(unit, concern)))
            .cloned()
            .collect()
    }

    fn clears(&self, unit: &Unit, concern: &Concern) -> bool {
        unit.moves
            .iter()
            .any(|lock_move| concern.is_cleared_by(lock_move) && !self.held(unit, lock_move))
    }

    fn held(&self, unit: &Unit, lock_move: &LockMove) -> bool {
        self.dependents.holding(unit).iter().any(|dependent| {
            dependent.requires == lock_move.name
                && lock_move.from.as_ref() == Some(&dependent.in_use)
                && lock_move.to.as_ref() == Some(&dependent.target)
        })
    }

    fn held_back(&self, units: &[Unit], concern: &Concern) -> Vec<LockMove> {
        let mut held = units
            .iter()
            .flat_map(|unit| {
                unit.moves
                    .iter()
                    .filter(|lock_move| concern.is_cleared_by(lock_move))
                    .filter(|lock_move| self.held(unit, lock_move))
                    .cloned()
            })
            .collect::<Vec<LockMove>>();
        held.dedup();
        held
    }
}

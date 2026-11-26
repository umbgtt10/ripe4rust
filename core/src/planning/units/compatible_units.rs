// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::concern::Concern;
use crate::planning::units::dependency_moves::DependencyMoves;
use crate::planning::units::direct_dependency::DirectDependency;
use crate::planning::units::unit::Unit;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// The compatible units, taken by crate name: a move several share belongs
/// to the first, the moves no unit reaches form the rest, and a move of a
/// crate pinned with `hold = "all"` is held out of every unit -- unless it
/// clears an advisory or a yanked release, which no pin holds. A unit whose
/// own move is held takes its crate to no version; one whose crate does not
/// move keeps its version.
pub struct CompatibleUnits {
    held: Vec<String>,
    concerns: Vec<Concern>,
}

impl CompatibleUnits {
    pub fn new(held: Vec<String>, concerns: Vec<Concern>) -> Self {
        Self { held, concerns }
    }

    pub fn build(&self, dependencies: Vec<DependencyMoves>, workspace: Vec<LockMove>) -> Vec<Unit> {
        let mut claimed: Vec<LockMove> = Vec::new();
        let mut units = Vec::new();
        for each in dependencies {
            let fresh = Self::unclaimed(each.moves, &claimed);
            claimed.extend(fresh.iter().cloned());
            if !fresh.is_empty() {
                units.push(self.unit(each.dependency, &fresh));
            }
        }
        let rest = Self::unclaimed(workspace, &claimed);
        if !rest.is_empty() {
            units.push(Unit {
                id: UnitId::rest(),
                kind: UnitKind::Compatible,
                name: None,
                members: Vec::new(),
                from: None,
                to: None,
                moves: self.kept(&rest, false),
                held: self.kept(&rest, true),
            });
        }
        units
    }

    pub fn holds(&self, lock_move: &LockMove) -> bool {
        self.held.contains(&lock_move.name)
            && !self
                .concerns
                .iter()
                .any(|concern| concern.is_cleared_by(lock_move))
    }

    fn kept(&self, moves: &[LockMove], held: bool) -> Vec<LockMove> {
        moves
            .iter()
            .filter(|lock_move| self.holds(lock_move) == held)
            .cloned()
            .collect()
    }

    fn unclaimed(moves: Vec<LockMove>, claimed: &[LockMove]) -> Vec<LockMove> {
        moves
            .into_iter()
            .filter(|lock_move| {
                !claimed
                    .iter()
                    .any(|taken| taken.name == lock_move.name && taken.from == lock_move.from)
            })
            .collect()
    }

    fn unit(&self, dependency: DirectDependency, moves: &[LockMove]) -> Unit {
        let kept = self.kept(moves, false);
        let held = self.kept(moves, true);
        let own = |moves: &[LockMove]| {
            moves
                .iter()
                .find(|lock_move| lock_move.name == dependency.name)
                .map(|lock_move| lock_move.to.clone())
        };
        let to = match (own(&kept), own(&held)) {
            (Some(to), _) => to,
            (None, Some(_)) => None,
            (None, None) => Some(dependency.version.clone()),
        };
        Unit {
            id: UnitId::of(UnitKind::Compatible, &dependency.name),
            kind: UnitKind::Compatible,
            name: Some(dependency.name.clone()),
            members: vec![dependency.name],
            from: Some(dependency.version),
            to,
            moves: kept,
            held,
        }
    }
}

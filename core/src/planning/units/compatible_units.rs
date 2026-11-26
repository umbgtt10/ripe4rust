// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::claimant::Claimant;
use crate::planning::units::concern::Concern;
use crate::planning::units::dependency_moves::DependencyMoves;
use crate::planning::units::direct_dependency::DirectDependency;
use crate::planning::units::source_moves::SourceMoves;
use crate::planning::units::unit::Unit;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// The compatible units, taken by crate name, and the git units among them,
/// each a source moving with what it pulls in: a move several share belongs
/// to the first, the moves no unit reaches form the rest, and a move of a
/// crate pinned with `hold = "all"` is held out of every unit -- unless it
/// clears an advisory or a yanked release, which no pin holds. A unit whose
/// own move is held, or whose crate does not move, only what it pulls in,
/// takes its crate to no version.
pub struct CompatibleUnits {
    held: Vec<String>,
    concerns: Vec<Concern>,
}

impl CompatibleUnits {
    pub fn new(held: Vec<String>, concerns: Vec<Concern>) -> Self {
        Self { held, concerns }
    }

    pub fn build(&self, dependencies: Vec<DependencyMoves>, workspace: Vec<LockMove>) -> Vec<Unit> {
        self.build_with_sources(dependencies, Vec::new(), workspace)
    }

    pub fn build_with_sources(
        &self,
        dependencies: Vec<DependencyMoves>,
        sources: Vec<SourceMoves>,
        workspace: Vec<LockMove>,
    ) -> Vec<Unit> {
        let mut claimants = dependencies
            .into_iter()
            .map(Claimant::Dependency)
            .chain(sources.into_iter().map(Claimant::Source))
            .collect::<Vec<Claimant>>();
        claimants.sort_by(|left, right| left.name().cmp(right.name()));
        let mut claimed: Vec<LockMove> = Vec::new();
        let mut units = Vec::new();
        for each in claimants {
            let fresh = Self::unclaimed(each.moves().to_vec(), &claimed);
            claimed.extend(fresh.iter().cloned());
            match each {
                Claimant::Dependency(dependency) if !fresh.is_empty() => {
                    units.push(self.unit(dependency.dependency, &fresh));
                }
                Claimant::Dependency(_) => {}
                Claimant::Source(source) => units.push(Self::git(source, fresh)),
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
                git: None,
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

    fn git(source: SourceMoves, moves: Vec<LockMove>) -> Unit {
        Unit {
            id: UnitId::of(UnitKind::Git, &source.name),
            kind: UnitKind::Git,
            name: Some(source.name),
            members: source.crates,
            from: None,
            to: None,
            moves,
            held: Vec::new(),
            git: Some(source.source),
        }
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
        let to = own(&kept).flatten();
        Unit {
            id: UnitId::of(UnitKind::Compatible, &dependency.name),
            kind: UnitKind::Compatible,
            name: Some(dependency.name.clone()),
            members: vec![dependency.name],
            from: Some(dependency.version),
            to,
            moves: kept,
            held,
            git: None,
        }
    }
}

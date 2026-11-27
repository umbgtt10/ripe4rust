// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::inputs::workspace::Workspace;
use crate::planning::judging::dependent::Dependent;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec::Vec;

/// What holds a breaking unit's crate where it is: every package of the
/// lock, the unit's own members aside, that depends on the version in use
/// with a requirement the unit's target does not meet. Moving the crate
/// then adds the target beside the version in use, which stays for them.
/// An advisory row's are those holding back the moves that would clear it.
pub struct Dependents<'a> {
    workspace: &'a Workspace,
}

impl<'a> Dependents<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }

    pub fn holding(&self, unit: &Unit) -> Vec<Dependent> {
        let moves = match unit.kind {
            UnitKind::Breaking => &unit.moves,
            UnitKind::Advisory => &unit.held,
            _ => return Vec::new(),
        };
        let mut holding = moves
            .iter()
            .flat_map(|lock_move| self.of(lock_move, &unit.members))
            .collect::<Vec<Dependent>>();
        holding.sort_by(|left, right| {
            (&left.requires, &left.name, &left.version).cmp(&(
                &right.requires,
                &right.name,
                &right.version,
            ))
        });
        holding.dedup();
        holding
    }

    fn of(&self, lock_move: &LockMove, members: &[String]) -> Vec<Dependent> {
        let (Some(from), Some(to)) = (&lock_move.from, &lock_move.to) else {
            return Vec::new();
        };
        self.workspace
            .lock
            .iter()
            .filter(|package| !members.contains(&package.name))
            .flat_map(|package| {
                package
                    .dependencies
                    .iter()
                    .filter(|dependency| {
                        dependency.name == lock_move.name
                            && dependency.version == *from
                            && !dependency.requirement.matches(to)
                    })
                    .map(|dependency| Dependent {
                        name: package.name.clone(),
                        version: package.version.clone(),
                        requires: dependency.name.clone(),
                        requirement: dependency.requirement.clone(),
                        in_use: from.clone(),
                        target: to.clone(),
                    })
            })
            .collect()
    }
}

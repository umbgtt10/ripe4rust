// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::workspace::Workspace;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;

/// Whether migrating a breaking unit removes a second copy of a crate: the
/// lock already holds it on the target's line, and nothing outside the
/// workspace's members depends on the copy in use.
pub struct Copies<'a> {
    workspace: &'a Workspace,
}

impl<'a> Copies<'a> {
    pub fn new(workspace: &'a Workspace) -> Self {
        Self { workspace }
    }

    pub fn removed(&self, unit: &Unit) -> bool {
        unit.kind == UnitKind::Breaking
            && unit.moves.iter().any(|lock_move| {
                let (Some(from), Some(to)) = (&lock_move.from, &lock_move.to) else {
                    return false;
                };
                let other_copy = self.workspace.lock.iter().any(|package| {
                    package.name == lock_move.name && package.version.line() == to.line()
                });
                let used_beyond = self.workspace.lock.iter().any(|package| {
                    package.dependencies.iter().any(|dependency| {
                        dependency.name == lock_move.name && dependency.version == *from
                    })
                });
                other_copy && !used_beyond
            })
    }
}

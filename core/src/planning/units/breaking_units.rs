// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::choice::breaking_choice::BreakingChoice;
use crate::planning::choice::histories::Histories;
use crate::planning::choice::maturity::Maturity;
use crate::planning::units::direct_dependency::DirectDependency;
use crate::planning::units::unit::Unit;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::vec;
use alloc::vec::Vec;

/// A breaking unit for every direct dependency with a newer line the
/// toolchain can build: the crate and its target, what the lock moves with
/// it left for `check` to report.
pub struct BreakingUnits<'a> {
    histories: &'a Histories,
    maturity: &'a Maturity,
    toolchain: &'a Version,
}

impl<'a> BreakingUnits<'a> {
    pub fn new(histories: &'a Histories, maturity: &'a Maturity, toolchain: &'a Version) -> Self {
        Self {
            histories,
            maturity,
            toolchain,
        }
    }

    pub fn build(&self, dependencies: &[DirectDependency]) -> Vec<Unit> {
        dependencies
            .iter()
            .filter_map(|dependency| {
                let history = self.histories.get(&dependency.name)?;
                let target = BreakingChoice::new(history, self.maturity, self.toolchain)
                    .choose(&dependency.version)?;
                Some(Unit {
                    id: UnitId::of(UnitKind::Breaking, &dependency.name),
                    kind: UnitKind::Breaking,
                    name: Some(dependency.name.clone()),
                    members: vec![dependency.name.clone()],
                    from: Some(dependency.version.clone()),
                    to: Some(target.to.clone()),
                    moves: vec![LockMove {
                        name: dependency.name.clone(),
                        from: Some(dependency.version.clone()),
                        to: Some(target.to),
                    }],
                    held: Vec::new(),
                })
            })
            .collect()
    }
}

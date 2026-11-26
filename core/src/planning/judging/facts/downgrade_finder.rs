// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::downgrade::Downgrade;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::units::unit::Unit;
use alloc::vec::Vec;

/// The moves inside a unit to a lower version of a crate, in the unit's
/// order, each saying whether the version it leaves is yanked.
pub struct DowngradeFinder<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> DowngradeFinder<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn of(&self, unit: &Unit) -> Vec<Downgrade> {
        unit.moves
            .iter()
            .filter_map(|lock_move| {
                let (Some(from), Some(to)) = (&lock_move.from, &lock_move.to) else {
                    return None;
                };
                (to < from).then(|| Downgrade {
                    name: lock_move.name.clone(),
                    from: from.clone(),
                    to: to.clone(),
                    yanked: self.context.histories().is_yanked(&lock_move.name, from),
                })
            })
            .collect()
    }
}

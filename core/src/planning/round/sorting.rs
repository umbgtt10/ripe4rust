// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::inputs::recorded_plan::RecordedPlan;
use crate::inputs::recorded_unit::RecordedUnit;
use crate::planning::round::current_state::CurrentState;
use crate::planning::round::line_change::LineChange;
use crate::planning::round::line_changes::LineChanges;
use crate::planning::round::lock_changes::LockChanges;
use crate::planning::round::progress::Progress;
use crate::planning::round::sorted::Sorted;
use crate::planning::round::stale_findings::StaleFindings;
use crate::unit_kind::UnitKind;
use alloc::vec::Vec;

/// Every difference between the workspace the plan found and the workspace
/// now, sorted into the units that explain it. A lock move a unit of the plan
/// makes is that unit's; a line rewritten to the value a unit's edit writes is
/// that unit's; any other line is a change outside the plan.
pub struct Sorting<'a> {
    plan: &'a RecordedPlan,
    lock: LockChanges,
    lines: LineChanges,
}

impl<'a> Sorting<'a> {
    pub fn new(plan: &'a RecordedPlan, current: &CurrentState) -> Self {
        Self {
            plan,
            lock: LockChanges::new(&plan.lock, &current.lock),
            lines: LineChanges::new(&plan.lines, &current.lines),
        }
    }

    pub fn lines(&self) -> &LineChanges {
        &self.lines
    }

    pub fn lock(&self) -> &LockChanges {
        &self.lock
    }

    pub fn sort(&self) -> Sorted {
        let progress = Progress::new(&self.lock, &self.lines);
        let units = self
            .plan
            .units
            .iter()
            .filter(|unit| unit.kind != UnitKind::Advisory)
            .collect::<Vec<&RecordedUnit>>();
        let moved = units
            .iter()
            .flat_map(|unit| unit.moves.iter())
            .filter(|lock_move| self.lock.applied(lock_move))
            .cloned()
            .collect::<Vec<LockMove>>();
        Sorted {
            applied: units
                .iter()
                .flat_map(|unit| progress.applied(unit))
                .collect(),
            to_go: units.iter().flat_map(|unit| progress.to_go(unit)).collect(),
            breaking: units
                .iter()
                .filter(|unit| unit.kind == UnitKind::Breaking && progress.whole(unit))
                .map(|unit| unit.id.clone())
                .collect(),
            unexplained: self.lock.unexplained(&moved),
            stale: self
                .lines
                .changes()
                .iter()
                .filter(|change| !self.edited(change))
                .map(|change| StaleFindings.line(change))
                .collect(),
        }
    }

    fn edited(&self, change: &LineChange) -> bool {
        let LineChange::Rewritten { then, now } = change else {
            return false;
        };
        self.plan
            .units
            .iter()
            .flat_map(|unit| unit.edits.iter())
            .any(|edit| edit.at == then.at && edit.key == then.key && edit.value == now.value)
    }
}

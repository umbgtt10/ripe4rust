// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::recorded_plan::RecordedPlan;
use crate::planning::round::target::Target;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// The unit an id names: one of the plan's units, or one move of a unit's
/// split, `<unit>/<crate>`; any other id is `unit-unknown`.
pub struct TargetFinder<'a> {
    plan: &'a RecordedPlan,
}

impl<'a> TargetFinder<'a> {
    pub fn new(plan: &'a RecordedPlan) -> Self {
        Self { plan }
    }

    pub fn find(&self, id: &str) -> Result<Target, Diagnostic> {
        self.unit(id).or_else(|| self.split(id)).ok_or_else(|| {
            DiagnosticCode::UnitUnknown.diagnose(
                None,
                format!("`{id}` is no unit of the plan, nor a move one of its units splits into"),
                String::from(
                    "name a unit by the `id` the plan gives it, or one of its moves by an `id` of its `steps.split`",
                ),
            )
        })
    }

    fn split(&self, id: &str) -> Option<Target> {
        self.plan
            .units
            .iter()
            .filter(|unit| unit.splits.iter().any(|split| split == id))
            .find_map(|unit| {
                let name = id.strip_prefix(&unit.id)?.strip_prefix('/')?;
                let lock_move = unit.moves.iter().find(|lock_move| lock_move.name == name)?;
                Some(Target {
                    id: String::from(id),
                    unit: unit.clone(),
                    moves: vec![lock_move.clone()],
                    edits: Vec::new(),
                })
            })
    }

    fn unit(&self, id: &str) -> Option<Target> {
        self.plan
            .units
            .iter()
            .find(|unit| unit.id == id)
            .map(|unit| Target {
                id: String::from(id),
                unit: unit.clone(),
                moves: unit.moves.clone(),
                edits: unit.edits.clone(),
            })
    }
}

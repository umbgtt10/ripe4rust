// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::planning::planned_unit::PlannedUnit;
use crate::planning::workspace_state::WorkspaceState;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec::Vec;

/// The plan: one row per unit, in plan order, with the date it was made
/// for, the day its registry answers are from, the workspace it was made
/// from, the gates and every problem met on the way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub date: Date,
    pub registry_read: Date,
    pub workspace: WorkspaceState,
    pub gates: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
    pub units: Vec<PlannedUnit>,
}

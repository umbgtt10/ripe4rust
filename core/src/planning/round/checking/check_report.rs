// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::inputs::lock_move::LockMove;
use crate::planning::round::checking::difference::Difference;
use alloc::string::String;
use alloc::vec::Vec;

/// What `check` found of a unit: whether it is as planned, the moves a
/// breaking unit's edit made beyond the plan's, every difference with what to
/// do, the units applied and those still to go, and the policy's warnings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckReport {
    pub unit: String,
    pub as_planned: bool,
    pub transitive: Vec<LockMove>,
    pub differences: Vec<Difference>,
    pub applied: Vec<String>,
    pub to_go: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
}

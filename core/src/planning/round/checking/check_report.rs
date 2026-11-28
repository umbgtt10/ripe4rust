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
/// do, the moves of crates the units share that came out otherwise, the
/// dependencies re-pointed between versions the lock already held -- told,
/// never a difference --, the units applied and those still to go, and the
/// policy's warnings with the check's own notices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckReport {
    pub unit: String,
    pub as_planned: bool,
    pub transitive: Vec<LockMove>,
    pub differences: Vec<Difference>,
    pub shared: Vec<String>,
    pub repointed: Vec<String>,
    pub applied: Vec<String>,
    pub to_go: Vec<String>,
    pub diagnostics: Vec<Diagnostic>,
}

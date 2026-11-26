// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::inputs::lock_move::LockMove;
use alloc::string::String;
use alloc::vec::Vec;

/// Every difference from the workspace the plan found, sorted: the units and
/// split moves applied and those still to go, in plan order; the breaking
/// units applied; the lock's changes no move of the plan accounts for; and a
/// `plan-stale` diagnostic for every line no unit's edit explains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sorted {
    pub applied: Vec<String>,
    pub to_go: Vec<String>,
    pub breaking: Vec<String>,
    pub unexplained: Vec<LockMove>,
    pub stale: Vec<Diagnostic>,
}

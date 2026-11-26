// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use alloc::vec::Vec;

/// No plan could be made: the diagnostics say why, the errors among them
/// first to act on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlanFailure {
    pub diagnostics: Vec<Diagnostic>,
}

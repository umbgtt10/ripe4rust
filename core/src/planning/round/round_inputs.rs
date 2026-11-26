// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::planning::pins_and_policy::policy::Policy;
use crate::planning::round::current_state::CurrentState;
use alloc::vec::Vec;

/// What a round's command reads of the workspace beside the plan: the
/// policy as it stands, the workspace now, and the policy's warnings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoundInputs {
    pub policy: Policy,
    pub current: CurrentState,
    pub diagnostics: Vec<Diagnostic>,
}

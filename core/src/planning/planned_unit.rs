// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::decision::Decision;
use crate::planning::units::unit::Unit;
use crate::planning::writing::evidence::Evidence;
use crate::planning::writing::question::Question;
use crate::planning::writing::steps::Steps;
use alloc::string::String;

/// One row of the plan: the unit, what was decided and why, its steps --
/// none for an advisory row -- and, where it asks, its question.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedUnit {
    pub unit: Unit,
    pub decision: Decision,
    pub summary: String,
    pub steps: Option<Steps>,
    pub question: Option<Question>,
    pub evidence: Evidence,
}

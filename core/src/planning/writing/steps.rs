// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::writing::revert::Revert;
use crate::planning::writing::split_step::SplitStep;
use crate::planning::writing::step::Step;
use alloc::string::String;
use alloc::vec::Vec;

/// What an agent with no context does with a unit: apply it, check it, run
/// the gates, revert it, and bisect a compatible one through `split`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Steps {
    pub apply: Vec<Step>,
    pub check: String,
    pub gates: Vec<String>,
    pub revert: Revert,
    pub split: Vec<SplitStep>,
}

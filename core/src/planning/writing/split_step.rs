// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::writing::step::Step;
use alloc::string::String;
use alloc::vec::Vec;

/// One move of a compatible unit, applied alone, for bisecting it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitStep {
    pub id: String,
    pub apply: Vec<Step>,
}

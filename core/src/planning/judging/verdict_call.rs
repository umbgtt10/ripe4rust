// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::suggestion::Suggestion;
use alloc::string::String;

/// The verdict on a breaking unit that asks, and its reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdictCall {
    pub suggestion: Suggestion,
    pub reason: String,
}

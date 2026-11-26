// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::suggestion::Suggestion;
use alloc::string::String;
use alloc::vec::Vec;

/// What the rules decided of a unit: the rule that matched (none for an
/// advisory row), the short reasons a table shows and, for a unit that asks,
/// why, in a sentence ready for the question.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Decision {
    pub outcome: Outcome,
    pub urgent: bool,
    pub rule: Option<u8>,
    pub suggestion: Option<Suggestion>,
    pub reasons: Vec<String>,
    pub why: Option<String>,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::writing::answer::Answer;
use alloc::string::String;

/// One answer to a question: what it means, and the step that carries it out
/// without asking again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestionOption {
    pub answer: Answer,
    pub means: String,
    pub then: String,
}

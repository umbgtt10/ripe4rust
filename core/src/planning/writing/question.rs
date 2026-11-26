// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::writing::answer::Answer;
use crate::planning::writing::question_option::QuestionOption;
use alloc::string::String;
use alloc::vec::Vec;

/// The question a unit that asks puts to the person, ready to present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Question {
    pub title: String,
    pub why: String,
    pub recommendation: Answer,
    pub options: Vec<QuestionOption>,
}

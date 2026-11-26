// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::option_document::OptionDocument;
use crate::render::json::documents::question_document::QuestionDocument;
use ripe4rust_core::planning::writing::question::Question;

/// The question of a unit that asks, every option with its `then`.
pub struct QuestionJson;

impl QuestionJson {
    pub fn document<'a>(&self, question: &'a Question) -> QuestionDocument<'a> {
        QuestionDocument {
            title: &question.title,
            why: &question.why,
            recommendation: question.recommendation.label(),
            options: question
                .options
                .iter()
                .map(|option| OptionDocument {
                    answer: option.answer.label(),
                    means: &option.means,
                    then: &option.then,
                })
                .collect(),
        }
    }
}

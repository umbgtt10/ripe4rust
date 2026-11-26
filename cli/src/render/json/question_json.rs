// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::writing::question::Question;
use serde_json::Value;
use serde_json::json;

/// The question of a unit that asks, every option with its `then`.
pub struct QuestionJson;

impl QuestionJson {
    pub fn value(&self, question: &Question) -> Value {
        json!({
            "title": question.title,
            "why": question.why,
            "recommendation": question.recommendation.label(),
            "options": question
                .options
                .iter()
                .map(|option| json!({
                    "answer": option.answer.label(),
                    "means": option.means,
                    "then": option.then,
                }))
                .collect::<Vec<Value>>(),
        })
    }
}

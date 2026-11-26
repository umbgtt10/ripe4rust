// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::value;
use crate::support::plans::breaking;
use cargo_ripe4rust::render::json::question_json::QuestionJson;
use serde_json::json;

#[test]
fn document_of_a_question_carries_every_option_s_then() {
    // Arrange
    let question = breaking().question.expect("a question");

    // Act
    let value = value(&QuestionJson.document(&question));

    // Assert
    assert_eq!(
        value,
        json!({
            "title": "Move gadget to 0.14.0?",
            "why": "it reaches thumbv7em-none-eabihf, which the gates cannot validate",
            "recommendation": "plan-it",
            "options": [{
                "answer": "wait",
                "means": "not now",
                "then": "cargo ripe4rust hold breaking:gadget --plan plan.json --because \"<the person's words>\"",
            }],
        })
    );
}

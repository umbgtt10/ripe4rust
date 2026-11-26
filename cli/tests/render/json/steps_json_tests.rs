// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::value;
use crate::support::plans::compatible;
use cargo_ripe4rust::render::json::steps_json::StepsJson;
use serde_json::json;

#[test]
fn document_of_a_compatible_unit_s_steps_is_the_design_s_shape() {
    // Arrange
    let steps = compatible().steps.expect("steps");

    // Act
    let value = value(&StepsJson.document(&steps));

    // Assert
    assert_eq!(
        value,
        json!({
            "apply": [{ "run": "cargo update --recursive -p widget" }],
            "check": "cargo ripe4rust check compatible:widget --plan plan.json",
            "gates": ["just stage1", "just stage2"],
            "revert": { "restore": ["Cargo.lock"], "run": "git restore -- Cargo.lock" },
            "split": [
                { "id": "compatible:widget/widget", "apply": [{ "run": "cargo update -p widget --precise 4.6.7" }] }
            ],
        })
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::at;
use crate::support::plans::run;
use cargo_ripe4rust::render::json::step_json::StepJson;
use ripe4rust_core::planning::writing::step::Step;
use serde_json::json;

#[test]
fn value_of_a_command_is_run() {
    // Arrange & Act
    let value = StepJson.value(&run("cargo update --recursive -p widget"));

    // Assert
    assert_eq!(
        value,
        json!({ "run": "cargo update --recursive -p widget" })
    );
}

#[test]
fn value_of_an_edit_names_the_line_the_key_and_the_value() {
    // Arrange
    let edit = Step::Edit {
        at: at("Cargo.toml", 22),
        key: String::from("gadget"),
        value: String::from("0.14.0"),
    };

    // Act
    let value = StepJson.value(&edit);

    // Assert
    assert_eq!(
        value,
        json!({ "edit": "Cargo.toml:22", "key": "gadget", "value": "0.14.0" })
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::value;
use crate::support::plans::compatible;
use crate::support::plans::moved;
use crate::support::plans::rest;
use cargo_ripe4rust::render::json::unit_json::UnitJson;
use serde_json::json;

#[test]
fn document_of_a_compatible_unit_is_the_design_s_shape() {
    // Arrange & Act
    let value = value(&UnitJson.document(&compatible()));

    // Assert
    assert_eq!(
        value,
        json!({
            "id": "compatible:widget",
            "kind": "compatible",
            "crate": "widget",
            "members": ["widget"],
            "from": "4.6.6",
            "to": "4.6.7",
            "moves": [
                { "crate": "widget", "from": "4.6.6", "to": "4.6.7" },
                { "crate": "widget_builder", "from": "4.6.6", "to": "4.6.7" },
            ],
            "outcome": "ripe",
            "urgent": false,
            "rule": 10,
            "suggestion": null,
            "reasons": ["compatible"],
            "summary": "Move widget to 4.6.7, with 1 crate it pulls in",
            "steps": {
                "apply": [{ "run": "cargo update --recursive -p widget" }],
                "check": "cargo ripe4rust check compatible:widget --plan plan.json",
                "gates": ["just stage1", "just stage2"],
                "revert": { "restore": ["Cargo.lock"], "run": "git restore -- Cargo.lock" },
                "split": [
                    { "id": "compatible:widget/widget", "apply": [{ "run": "cargo update -p widget --precise 4.6.7" }] }
                ],
            },
            "question": null,
            "evidence": {
                "line_in_use": { "line": "4.x", "first_release": "2022-01-01", "last_release": "2026-10-02", "releases": 41 },
                "rust_version": "1.74",
                "advisories": [],
                "yanked_in_use": false,
                "footprint": [],
                "held_back": [],
                "held": [],
            },
        })
    );
}

#[test]
fn document_of_a_unit_whose_own_move_is_held_takes_no_version() {
    // Arrange
    let mut held = compatible();
    held.unit.to = None;
    held.unit.moves = vec![moved("widget_builder", Some("4.6.6"), Some("4.6.7"))];
    held.evidence.held = vec![moved("widget", Some("4.6.6"), Some("4.6.7"))];

    // Act
    let value = value(&UnitJson.document(&held));

    // Assert
    assert_eq!(value["from"], json!("4.6.6"));
    assert_eq!(value["to"], json!(null));
    assert_eq!(
        value["evidence"]["held"],
        json!([{ "crate": "widget", "from": "4.6.6", "to": "4.6.7" }])
    );
}

#[test]
fn document_of_the_rest_has_no_crate() {
    // Arrange & Act
    let value = value(&UnitJson.document(&rest()));

    // Assert
    assert_eq!(value["id"], json!("rest"));
    assert_eq!(value["crate"], json!(null));
    assert_eq!(value["from"], json!(null));
}

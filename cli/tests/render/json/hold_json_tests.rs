// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::value;
use crate::support::reports::held;
use cargo_ripe4rust::render::json::hold_json::HoldJson;
use serde_json::json;

#[test]
fn document_of_pins_names_the_section_its_line_and_what_to_do() {
    // Arrange
    let report = held(true);

    // Act
    let document = HoldJson.document(&report);

    // Assert
    assert_eq!(
        value(&document),
        json!({
            "unit": "compatible:widget",
            "section": "[workspace.metadata.ripe4rust.pins]",
            "where": "Cargo.toml:40",
            "pins": [
                {
                    "crate": "widget",
                    "line": "widget = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }",
                    "replaces": "Cargo.toml:41"
                },
                {
                    "crate": "gizmo",
                    "line": "gizmo = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }",
                    "replaces": null
                }
            ],
            "do": "put the pins there, and commit it",
            "diagnostics": []
        })
    );
}

#[test]
fn document_of_pins_with_no_table_has_no_where() {
    // Arrange
    let report = held(false);

    // Act
    let document = value(&HoldJson.document(&report));

    // Assert
    assert_eq!(document["where"], json!(null));
    assert_eq!(document["pins"][0]["replaces"], json!(null));
}

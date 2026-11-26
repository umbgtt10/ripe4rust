// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::text;
use crate::support::json::value;
use crate::support::reports::as_planned;
use crate::support::reports::not_as_planned;
use cargo_ripe4rust::render::json::check_json::CheckJson;
use serde_json::json;

#[test]
fn document_of_a_report_as_planned_names_the_moves_its_edit_made() {
    // Arrange
    let report = as_planned();

    // Act
    let document = CheckJson.document(&report);

    // Assert
    assert_eq!(
        value(&document),
        json!({
            "unit": "breaking:gadget",
            "as_planned": true,
            "transitive": [
                { "crate": "gizmo", "from": "2.7.5", "to": "2.7.6" },
                { "crate": "tiny", "from": null, "to": "1.0.0" },
                { "crate": "old", "from": "1.0.0", "to": null }
            ],
            "differences": [],
            "applied": ["compatible:widget", "breaking:gadget"],
            "to_go": ["rest"],
            "diagnostics": []
        })
    );
}

#[test]
fn document_of_a_report_keeps_the_design_s_order() {
    // Arrange
    let report = not_as_planned();

    // Act
    let written = text(&CheckJson.document(&report));

    // Assert
    assert!(
        written.starts_with(
            r#"{"unit":"compatible:widget","as_planned":false,"transitive":[],"differences":[{"where":"Cargo.lock","what":"#
        ),
        "{written}"
    );
    assert!(
        written.ends_with(r#""applied":[],"to_go":["compatible:widget","rest"],"diagnostics":[]}"#),
        "{written}"
    );
}

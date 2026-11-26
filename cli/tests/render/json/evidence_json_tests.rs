// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::value;
use crate::support::plans::breaking;
use crate::support::plans::compatible;
use crate::support::plans::moved;
use cargo_ripe4rust::render::json::evidence_json::EvidenceJson;
use serde_json::json;

#[test]
fn document_of_a_breaking_unit_s_evidence_carries_its_work_list() {
    // Arrange
    let mut evidence = breaking().evidence;
    evidence.held = vec![moved("gizmo", Some("2.7.5"), Some("2.7.6"))];

    // Act
    let value = value(&EvidenceJson.document(&evidence));

    // Assert
    assert_eq!(
        value,
        json!({
            "line_in_use": null,
            "rust_version": "1.80",
            "advisories": [],
            "yanked_in_use": false,
            "footprint": ["src/net.rs:4"],
            "held_back": [],
            "held": [{ "crate": "gizmo", "from": "2.7.5", "to": "2.7.6" }],
        })
    );
}

#[test]
fn document_of_the_line_in_use_names_its_releases_and_dates() {
    // Arrange & Act
    let value = value(&EvidenceJson.document(&compatible().evidence));

    // Assert
    assert_eq!(
        value["line_in_use"],
        json!({
            "line": "4.x",
            "first_release": "2022-01-01",
            "last_release": "2026-10-02",
            "releases": 41,
        })
    );
}

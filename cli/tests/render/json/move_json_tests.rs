// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::text;
use crate::support::json::value;
use crate::support::plans::moved;
use cargo_ripe4rust::render::json::move_json::MoveJson;
use serde_json::json;

#[test]
fn document_of_an_added_crate_has_no_from() {
    // Arrange & Act
    let value = value(&MoveJson.document(&moved("windows-link", None, Some("0.2.1"))));

    // Assert
    assert_eq!(
        value,
        json!({ "crate": "windows-link", "from": null, "to": "0.2.1" })
    );
}

#[test]
fn document_of_an_update_is_written_crate_from_to() {
    // Arrange & Act
    let written = text(&MoveJson.document(&moved("widget", Some("4.6.6"), Some("4.6.7"))));

    // Assert
    assert_eq!(written, r#"{"crate":"widget","from":"4.6.6","to":"4.6.7"}"#);
}

#[test]
fn document_of_an_update_names_both_versions() {
    // Arrange & Act
    let value = value(&MoveJson.document(&moved("widget", Some("4.6.6"), Some("4.6.7"))));

    // Assert
    assert_eq!(
        value,
        json!({ "crate": "widget", "from": "4.6.6", "to": "4.6.7" })
    );
}

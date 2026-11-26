// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::moved;
use cargo_ripe4rust::render::json::move_json::MoveJson;
use serde_json::json;

#[test]
fn value_of_an_added_crate_has_no_from() {
    // Arrange & Act
    let value = MoveJson.value(&moved("windows-link", None, Some("0.2.1")));

    // Assert
    assert_eq!(
        value,
        json!({ "crate": "windows-link", "from": null, "to": "0.2.1" })
    );
}

#[test]
fn value_of_an_update_names_both_versions() {
    // Arrange & Act
    let value = MoveJson.value(&moved("widget", Some("4.6.6"), Some("4.6.7")));

    // Assert
    assert_eq!(
        value,
        json!({ "crate": "widget", "from": "4.6.6", "to": "4.6.7" })
    );
}

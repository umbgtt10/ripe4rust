// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::at;
use cargo_ripe4rust::render::json::diagnostic_json::DiagnosticJson;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use serde_json::json;

#[test]
fn value_of_a_diagnostic_names_where_what_and_do() {
    // Arrange
    let diagnostic = DiagnosticCode::PinWithoutReason.diagnose(
        Some(at("Cargo.toml", 41)),
        String::from("the pin on `syn` gives no reason"),
        String::from("add `reason = \"...\"` to it: why the crate is held"),
    );

    // Act
    let value = DiagnosticJson.value(&diagnostic);

    // Assert
    assert_eq!(
        value,
        json!({
            "severity": "error",
            "code": "pin-without-reason",
            "where": "Cargo.toml:41",
            "what": "the pin on `syn` gives no reason",
            "do": "add `reason = \"...\"` to it: why the crate is held",
            "needs_person": true,
        })
    );
}

#[test]
fn value_of_a_diagnostic_nowhere_has_a_null_where() {
    // Arrange
    let diagnostic =
        DiagnosticCode::ToolchainUnpinned.diagnose(None, String::from("x"), String::from("y"));

    // Act
    let value = DiagnosticJson.value(&diagnostic);

    // Assert
    assert_eq!(value["where"], json!(null));
    assert_eq!(value["severity"], json!("notice"));
}

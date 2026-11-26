// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::at;
use cargo_ripe4rust::render::table::diagnostic_line::DiagnosticLine;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;

#[test]
fn line_of_a_diagnostic_ending_in_a_full_stop_does_not_double_it() {
    // Arrange
    let diagnostic = DiagnosticCode::CargoMetadataFailed.diagnose(
        None,
        String::from("`cargo metadata` could not run: use --offline instead."),
        String::from("fix what stops it"),
    );

    // Act
    let line = DiagnosticLine.line(&diagnostic);

    // Assert
    assert_eq!(
        line,
        "error[cargo-metadata-failed] `cargo metadata` could not run: use --offline instead. Do: fix what stops it"
    );
}

#[test]
fn line_of_a_diagnostic_with_a_place_names_it() {
    // Arrange
    let diagnostic = DiagnosticCode::PinWithoutReason.diagnose(
        Some(at("Cargo.toml", 41)),
        String::from("the pin on `syn` gives no reason"),
        String::from("add a reason"),
    );

    // Act
    let line = DiagnosticLine.line(&diagnostic);

    // Assert
    assert_eq!(
        line,
        "error[pin-without-reason] Cargo.toml:41: the pin on `syn` gives no reason. Do: add a reason"
    );
}

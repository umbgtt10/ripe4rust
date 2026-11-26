// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::reports::held;
use cargo_ripe4rust::render::table::hold_table::HoldTable;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;

#[test]
fn text_of_pins_with_diagnostics_prints_them_after_them() {
    // Arrange
    let mut report = held(true);
    report.diagnostics = vec![DiagnosticCode::GatesUndeclared.diagnose(
        None,
        String::from("no gates"),
        String::from("declare them"),
    )];

    // Act
    let text = HoldTable.text(&report);

    // Assert
    assert!(
        text.ends_with("Do: put the pins there, and commit it\n\nwarning[gates-undeclared] no gates. Do: declare them\n"),
        "{text}"
    );
}

#[test]
fn text_of_pins_with_no_table_writes_the_table_s_header_first() {
    // Arrange
    let report = held(false);

    // Act
    let text = HoldTable.text(&report);

    // Assert
    assert_eq!(
        text,
        "hold compatible:widget: in a new [workspace.metadata.ripe4rust.pins] at the end of Cargo.toml\n[workspace.metadata.ripe4rust.pins]\nwidget = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }\ngizmo = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }\nDo: put the pins there, and commit it\n"
    );
}

#[test]
fn text_of_pins_with_their_table_names_its_line() {
    // Arrange
    let report = held(true);

    // Act
    let text = HoldTable.text(&report);

    // Assert
    assert_eq!(
        text,
        "hold compatible:widget: under [workspace.metadata.ripe4rust.pins] at Cargo.toml:40\nwidget = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }\ngizmo = { reason = \"red\", review = \"2026-12-10\", hold = \"all\" }\nDo: put the pins there, and commit it\n"
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::reports::as_planned;
use crate::support::reports::not_as_planned;
use cargo_ripe4rust::render::table::check_table::CheckTable;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;

#[test]
fn text_of_a_difference_ending_in_a_full_stop_does_not_double_it() {
    // Arrange
    let mut report = not_as_planned();
    report.differences[0].what = String::from("`libc` moved 0.2.177 -> 0.2.180.");

    // Act
    let text = CheckTable.text(&report);

    // Assert
    assert!(
        text.contains("Cargo.lock: `libc` moved 0.2.177 -> 0.2.180. Do: carry out"),
        "{text}"
    );
}

#[test]
fn text_of_a_report_as_planned_names_the_moves_its_edit_made() {
    // Arrange
    let report = as_planned();

    // Act
    let text = CheckTable.text(&report);

    // Assert
    assert_eq!(
        text,
        "check breaking:gadget: as planned\ntransitive: gizmo 2.7.5 -> 2.7.6, tiny 1.0.0 added, old 1.0.0 removed\napplied: compatible:widget, breaking:gadget\nto go: rest\n"
    );
}

#[test]
fn text_of_a_report_not_as_planned_lists_each_difference_with_what_to_do() {
    // Arrange
    let report = not_as_planned();

    // Act
    let text = CheckTable.text(&report);

    // Assert
    assert_eq!(
        text,
        "check compatible:widget: not as planned\nCargo.lock: `libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes. Do: carry out the unit's `revert`\nshared: `wasip2` 1.0.3 left the lock: the units share the crate, and its moves depend on the order they are applied in.\napplied: none\nto go: compatible:widget, rest\n"
    );
}

#[test]
fn text_of_a_report_with_a_re_point_names_it_after_the_shared_moves() {
    // Arrange
    let mut report = not_as_planned();
    report.repointed = vec![String::from(
        "`errno` 0.3.14's `windows-sys` 0.61.2 -> 0.52.0, both already in the lock",
    )];

    // Act
    let text = CheckTable.text(&report);

    // Assert
    assert!(
        text.contains("in.\nrepointed: `errno` 0.3.14's `windows-sys` 0.61.2 -> 0.52.0, both already in the lock.\napplied: none\n"),
        "{text}"
    );
}

#[test]
fn text_of_a_report_with_diagnostics_prints_them_after_it() {
    // Arrange
    let mut report = not_as_planned();
    report.diagnostics = vec![DiagnosticCode::GatesUndeclared.diagnose(
        None,
        String::from("no gates"),
        String::from("declare them"),
    )];

    // Act
    let text = CheckTable.text(&report);

    // Assert
    assert!(
        text.ends_with("to go: compatible:widget, rest\n\nwarning[gates-undeclared] no gates. Do: declare them\n"),
        "{text}"
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::pins_reader::PinsReader;

#[test]
fn read_of_no_pins_is_empty() {
    // Arrange & Act
    let reading = PinsReader.read(&[]);

    // Assert
    assert_eq!(reading.value, []);
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_of_pins_that_are_not_a_table_is_invalid() {
    // Arrange
    let entries = [entry("pins", text("syn"), 40)];

    // Act
    let reading = PinsReader.read(&entries);

    // Assert
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::PolicyInvalidValue]
    );
}

#[test]
fn read_of_two_pins_keeps_the_good_one_and_reports_the_other() {
    // Arrange
    let entries = [entry(
        "pins",
        PolicyValue::Table(vec![
            entry(
                "syn",
                PolicyValue::Table(vec![
                    entry("reason", text("a migration"), 41),
                    entry("review", text("2027-01-31"), 41),
                ]),
                41,
            ),
            entry(
                "tokio",
                PolicyValue::Table(vec![entry("review", text("2027-02-28"), 42)]),
                42,
            ),
        ]),
        40,
    )];

    // Act
    let reading = PinsReader.read(&entries);

    // Assert
    assert_eq!(
        reading
            .value
            .iter()
            .map(|pin| pin.name.clone())
            .collect::<Vec<_>>(),
        vec![String::from("syn")]
    );
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::PinWithoutReason]
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::at;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use crate::support::policy_trees::texts;
use crate::support::policy_trees::tree;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::policy_reader::PolicyReader;
use ripe4rust_core::values::date::Date;

fn codes(diagnostics: &[Diagnostic]) -> Vec<DiagnosticCode> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

#[test]
fn read_of_a_full_policy_reads_every_key() {
    // Arrange
    let policy = tree(vec![
        entry("gates", texts(&["just stage1", "just stage2"]), 31),
        entry("published", texts(&["mytool-core"]), 32),
        entry(
            "validate-beyond-gates",
            texts(&["thumbv7em-none-eabihf"]),
            33,
        ),
        entry("freeze-until", text("2027-01-15"), 34),
        entry(
            "exposure",
            PolicyValue::Table(vec![entry("serde", text("public"), 35)]),
            35,
        ),
        entry("young-days", PolicyValue::Integer(20), 36),
        entry(
            "pins",
            PolicyValue::Table(vec![entry(
                "syn",
                PolicyValue::Table(vec![
                    entry("reason", text("a migration"), 41),
                    entry("review", text("2027-01-31"), 41),
                ]),
                41,
            )]),
            40,
        ),
    ]);

    // Act
    let reading = PolicyReader.read(&policy);

    // Assert
    let read = reading.value;
    assert_eq!(
        read.gates,
        vec![String::from("just stage1"), String::from("just stage2")]
    );
    assert_eq!(read.published, vec![String::from("mytool-core")]);
    assert_eq!(
        read.validate_beyond_gates,
        vec![String::from("thumbv7em-none-eabihf")]
    );
    assert_eq!(
        read.freeze_until,
        Some(Date::parse("2027-01-15").expect("a date"))
    );
    assert_eq!(read.exposure.len(), 1);
    assert_eq!(read.thresholds.young_days, 20);
    assert_eq!(read.pins.len(), 1);
    assert_eq!(read.at, Some(at(30)));
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_of_a_misspelt_key_is_an_error_where_it_is() {
    // Arrange
    let policy = tree(vec![
        entry("gates", texts(&["just stage1"]), 31),
        entry("young-dyas", PolicyValue::Integer(20), 36),
    ]);

    // Act
    let reading = PolicyReader.read(&policy);

    // Assert
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::PolicyUnknownKey]
    );
    assert_eq!(reading.diagnostics[0].at, Some(at(36)));
}

#[test]
fn read_of_a_policy_with_bad_values_reports_every_one() {
    // Arrange
    let policy = tree(vec![
        entry("gates", text("just stage1"), 31),
        entry("published", PolicyValue::Boolean(true), 32),
        entry("validate-beyond-gates", PolicyValue::Integer(1), 33),
        entry("freeze-until", text("January"), 34),
    ]);

    // Act
    let reading = PolicyReader.read(&policy);

    // Assert
    assert_eq!(
        codes(&reading.diagnostics),
        vec![
            DiagnosticCode::PolicyInvalidValue,
            DiagnosticCode::PolicyInvalidValue,
            DiagnosticCode::PolicyInvalidValue,
            DiagnosticCode::PolicyInvalidValue,
            DiagnosticCode::GatesUndeclared,
        ]
    );
}

#[test]
fn read_of_empty_gates_warns_of_no_gates_where_they_are() {
    // Arrange
    let policy = tree(vec![entry("gates", texts(&[]), 31)]);

    // Act
    let reading = PolicyReader.read(&policy);

    // Assert
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::GatesUndeclared]
    );
    assert_eq!(reading.diagnostics[0].at, Some(at(31)));
}

#[test]
fn read_of_no_policy_takes_the_defaults_and_warns_of_no_gates() {
    // Arrange
    let policy = PolicyTree {
        at: None,
        entries: Vec::new(),
    };

    // Act
    let reading = PolicyReader.read(&policy);

    // Assert
    assert_eq!(reading.value.thresholds.small_footprint, 10);
    assert_eq!(reading.value.freeze_until, None);
    assert_eq!(
        reading.diagnostics,
        vec![Diagnostic {
            severity: Severity::Warning,
            code: DiagnosticCode::GatesUndeclared,
            at: None,
            what: String::from(
                "no `gates` are declared, so nothing can validate a unit: every unit that would be ripe is asked"
            ),
            action: String::from(
                "ask the owner for the commands that validate a change, and write them as `gates = [...]` in `[workspace.metadata.ripe4rust]`"
            ),
            needs_person: true,
        }]
    );
}

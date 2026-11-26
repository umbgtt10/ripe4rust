// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::at;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::pins_and_policy::pin_reader::PinReader;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::values::date::Date;

fn codes(diagnostics: &[Diagnostic]) -> Vec<DiagnosticCode> {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn pin(name: &str, keys: Vec<PolicyEntry>) -> PolicyEntry {
    entry(name, PolicyValue::Table(keys), 41)
}

#[test]
fn read_of_a_hold_neither_line_nor_all_is_invalid() {
    // Arrange
    let entry = pin(
        "memchr",
        vec![
            entry("reason", text("2.7.6 broke stage 2"), 41),
            entry("review", text("2026-12-10"), 41),
            entry("hold", text("everything"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::PolicyInvalidValue]
    );
}

#[test]
fn read_of_a_pin_that_is_not_a_table_is_invalid() {
    // Arrange
    let entry = entry("syn", text("a migration"), 41);

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::PolicyInvalidValue]
    );
}

#[test]
fn read_of_a_pin_with_a_bad_review_date_reports_the_date_alone() {
    // Arrange
    let entry = pin(
        "syn",
        vec![
            entry("reason", text("a migration"), 41),
            entry("review", text("soon"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::PolicyInvalidValue]
    );
}

#[test]
fn read_of_a_pin_with_a_checkable_condition_needs_no_review_date() {
    // Arrange
    let entry = pin(
        "probe-rs",
        vec![
            entry("reason", text("the rig's CLI"), 41),
            entry(
                "until",
                PolicyValue::Table(vec![
                    entry("crate", text("probe-rs-tools"), 41),
                    entry("version", text(">=0.32"), 41),
                ]),
                41,
            ),
            entry("hold", text("all"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    let pin = reading.value.expect("a pin");
    assert!(matches!(pin.until, Some(ReleaseCondition::Release { .. })));
    assert_eq!(pin.hold, Hold::All);
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_of_a_pin_with_a_condition_in_words_and_no_review_date_has_no_release() {
    // Arrange
    let entry = pin(
        "tokio",
        vec![
            entry("reason", text("load-tested first"), 41),
            entry("until", text("the load test passes"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        reading.diagnostics,
        vec![Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::PinWithoutRelease,
            at: Some(at(41)),
            what: String::from(
                "the pin on `tokio` has neither a review date nor a release condition that can be checked"
            ),
            action: String::from(
                "add `review = \"YYYY-MM-DD\"` to it, or `until = { crate = \"...\", version = \">=...\" }`: when to look at it again"
            ),
            needs_person: true,
        }]
    );
}

#[test]
fn read_of_a_pin_with_a_reason_and_a_review_date_holds_the_line() {
    // Arrange
    let entry = pin(
        "syn",
        vec![
            entry("reason", text("a migration of the visitors"), 41),
            entry("review", text("2027-01-31"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(
        reading.value,
        Some(Pin {
            name: String::from("syn"),
            reason: String::from("a migration of the visitors"),
            review: Some(Date::parse("2027-01-31").expect("a date")),
            until: None,
            hold: Hold::Line,
            at: at(41),
        })
    );
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_of_a_pin_with_an_empty_reason_has_no_reason() {
    // Arrange
    let entry = pin(
        "syn",
        vec![
            entry("reason", text(" "), 41),
            entry("review", text("2027-01-31"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(
        codes(&reading.diagnostics),
        vec![DiagnosticCode::PinWithoutReason]
    );
}

#[test]
fn read_of_a_pin_with_an_unknown_key_names_it() {
    // Arrange
    let entry = pin(
        "syn",
        vec![
            entry("reason", text("a migration"), 41),
            entry("reveiw", text("2027-01-31"), 41),
        ],
    );

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        codes(&reading.diagnostics),
        vec![
            DiagnosticCode::PolicyUnknownKey,
            DiagnosticCode::PinWithoutRelease
        ]
    );
}

#[test]
fn read_of_a_pin_without_a_reason_says_so() {
    // Arrange
    let entry = pin("syn", vec![entry("review", text("2027-01-31"), 41)]);

    // Act
    let reading = PinReader.read(&entry);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        reading.diagnostics,
        vec![Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::PinWithoutReason,
            at: Some(at(41)),
            what: String::from("the pin on `syn` gives no reason"),
            action: String::from("add `reason = \"...\"` to it: why the crate is held"),
            needs_person: true,
        }]
    );
}

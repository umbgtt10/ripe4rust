// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::planning::pins_and_policy::until_reader::UntilReader;
use ripe4rust_core::values::requirement::Requirement;

fn reader() -> UntilReader {
    UntilReader::new("the pin on `probe-rs`")
}

#[test]
fn read_of_a_crate_and_a_version_is_checkable() {
    // Arrange
    let until = entry(
        "until",
        PolicyValue::Table(vec![
            entry("crate", text("probe-rs-tools"), 44),
            entry("version", text(">=0.32"), 44),
        ]),
        44,
    );

    // Act
    let reading = reader().read(&until);

    // Assert
    assert_eq!(
        reading.value,
        Some(ReleaseCondition::Release {
            name: String::from("probe-rs-tools"),
            requirement: Requirement::parse(">=0.32").expect("a requirement"),
        })
    );
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_of_a_number_is_invalid() {
    // Arrange & Act
    let reading = reader().read(&entry("until", PolicyValue::Integer(32), 44));

    // Assert
    assert_eq!(reading.value, None);
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
fn read_of_a_table_with_an_unknown_key_names_it() {
    // Arrange
    let until = entry(
        "until",
        PolicyValue::Table(vec![
            entry("crate", text("probe-rs-tools"), 44),
            entry("version", text(">=0.32"), 44),
            entry("since", text("2026-01-01"), 44),
        ]),
        44,
    );

    // Act
    let reading = reader().read(&until);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::PolicyUnknownKey]
    );
}

#[test]
fn read_of_a_table_without_a_version_is_invalid() {
    // Arrange
    let until = entry(
        "until",
        PolicyValue::Table(vec![entry("crate", text("probe-rs-tools"), 44)]),
        44,
    );

    // Act
    let reading = reader().read(&until);

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.what.clone())
            .collect::<Vec<_>>(),
        vec![String::from(
            "`until` in the pin on `probe-rs` is not a table of `crate` and `version`"
        )]
    );
}

#[test]
fn read_of_empty_words_is_invalid() {
    // Arrange & Act
    let reading = reader().read(&entry("until", text("  "), 44));

    // Assert
    assert_eq!(reading.value, None);
    assert_eq!(reading.diagnostics.len(), 1);
}

#[test]
fn read_of_words_is_a_condition_in_words() {
    // Arrange & Act
    let reading = reader().read(&entry("until", text("the load test passes"), 44));

    // Assert
    assert_eq!(
        reading.value,
        Some(ReleaseCondition::Words(String::from(
            "the load test passes"
        )))
    );
}

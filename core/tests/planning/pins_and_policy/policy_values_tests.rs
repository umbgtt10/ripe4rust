// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::at;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use crate::support::policy_trees::texts;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::policy_values::PolicyValues;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::requirement::Requirement;

fn values() -> PolicyValues {
    PolicyValues::new("`[workspace.metadata.ripe4rust]`")
}

#[test]
fn count_of_a_negative_number_is_invalid() {
    // Arrange & Act
    let count = values().count(&entry("young-days", PolicyValue::Integer(-1), 12));

    // Assert
    assert_eq!(
        count.map_err(|diagnostic| diagnostic.code),
        Err(DiagnosticCode::PolicyInvalidValue)
    );
}

#[test]
fn count_of_a_whole_number_is_it() {
    // Arrange & Act
    let count = values().count(&entry("young-days", PolicyValue::Integer(45), 12));

    // Assert
    assert_eq!(count, Ok(45));
}

#[test]
fn count_of_text_is_invalid() {
    // Arrange & Act
    let count = values().count(&entry("young-days", text("thirty"), 12));

    // Assert
    assert!(count.is_err());
}

#[test]
fn date_of_a_civil_date_is_it() {
    // Arrange & Act
    let date = values().date(&entry("freeze-until", text("2027-01-15"), 14));

    // Assert
    assert_eq!(date, Ok(Date::parse("2027-01-15").expect("a date")));
}

#[test]
fn date_of_a_misspelt_date_is_invalid() {
    // Arrange & Act
    let date = values().date(&entry("freeze-until", text("2027-13-15"), 14));

    // Assert
    assert!(date.is_err());
}

#[test]
fn invalid_names_the_key_the_section_and_what_it_should_be() {
    // Arrange & Act
    let diagnostic = values().invalid(&entry("young-days", text("thirty"), 12), "a whole number");

    // Assert
    assert_eq!(
        diagnostic,
        Diagnostic {
            severity: Severity::Error,
            code: DiagnosticCode::PolicyInvalidValue,
            at: Some(at(12)),
            what: String::from(
                "`young-days` in `[workspace.metadata.ripe4rust]` is not a whole number"
            ),
            action: String::from("write a whole number for `young-days`"),
            needs_person: true,
        }
    );
}

#[test]
fn requirement_of_a_lower_bound_is_it() {
    // Arrange & Act
    let requirement = values().requirement(&entry("version", text(">=0.32"), 44));

    // Assert
    assert_eq!(
        requirement,
        Ok(Requirement::parse(">=0.32").expect("a requirement"))
    );
}

#[test]
fn requirement_of_words_is_invalid() {
    // Arrange & Act
    let requirement = values().requirement(&entry("version", text("the next one"), 44));

    // Assert
    assert!(requirement.is_err());
}

#[test]
fn table_of_a_list_is_invalid() {
    // Arrange
    let list = entry("pins", texts(&["syn"]), 40);

    // Act
    let table = values().table(&list);

    // Assert
    assert!(table.is_err());
}

#[test]
fn table_of_a_table_is_its_entries() {
    // Arrange
    let pins = entry(
        "pins",
        PolicyValue::Table(vec![entry("syn", text("x"), 41)]),
        40,
    );

    // Act
    let table = values().table(&pins);

    // Assert
    assert_eq!(table.map(<[_]>::len), Ok(1));
}

#[test]
fn text_of_a_number_is_invalid() {
    // Arrange & Act
    let read = values().text(&entry("reason", PolicyValue::Integer(3), 41));

    // Assert
    assert!(read.is_err());
}

#[test]
fn text_of_text_is_it() {
    // Arrange & Act
    let read = values().text(&entry("reason", text("a migration"), 41));

    // Assert
    assert_eq!(read, Ok(String::from("a migration")));
}

#[test]
fn texts_of_a_list_holding_a_number_is_invalid() {
    // Arrange
    let mixed = PolicyValue::List(vec![text("just stage1"), PolicyValue::Integer(2)]);

    // Act
    let read = values().texts(&entry("gates", mixed, 31));

    // Assert
    assert!(read.is_err());
}

#[test]
fn texts_of_a_list_of_texts_keeps_their_order() {
    // Arrange & Act
    let read = values().texts(&entry("gates", texts(&["just stage1", "just stage2"]), 31));

    // Assert
    assert_eq!(
        read,
        Ok(vec![
            String::from("just stage1"),
            String::from("just stage2")
        ])
    );
}

#[test]
fn texts_of_a_single_text_is_invalid() {
    // Arrange & Act
    let read = values().texts(&entry("gates", text("just stage1"), 31));

    // Assert
    assert!(read.is_err());
}

#[test]
fn unknown_names_every_key_not_known_where_it_is() {
    // Arrange
    let entries = vec![
        entry("gates", texts(&[]), 31),
        entry("young-dyas", PolicyValue::Integer(3), 32),
        entry("frozen", PolicyValue::Boolean(true), 33),
    ];

    // Act
    let unknown = values().unknown(&entries, &["gates"]);

    // Assert
    assert_eq!(
        unknown
            .iter()
            .map(|diagnostic| (
                diagnostic.code,
                diagnostic.at.clone(),
                diagnostic.what.clone()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                DiagnosticCode::PolicyUnknownKey,
                Some(at(32)),
                String::from(
                    "`young-dyas` is not a key ripe4rust knows in `[workspace.metadata.ripe4rust]`"
                )
            ),
            (
                DiagnosticCode::PolicyUnknownKey,
                Some(at(33)),
                String::from(
                    "`frozen` is not a key ripe4rust knows in `[workspace.metadata.ripe4rust]`"
                )
            ),
        ]
    );
}

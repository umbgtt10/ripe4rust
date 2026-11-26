// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::round::guiding::fields_guide::FieldsGuide;

#[test]
fn fields_belong_to_the_documents_the_commands_print_each_saying_what_it_is() {
    // Arrange & Act
    let fields = FieldsGuide.fields();

    // Assert
    let mut documents = fields
        .iter()
        .map(|field| field.document.as_str())
        .collect::<Vec<&str>>();
    documents.dedup();
    assert_eq!(documents, vec!["every", "plan", "check", "hold", "guide"]);
    assert!(fields.iter().all(|field| !field.means.is_empty()));
}

#[test]
fn fields_of_a_unit_s_target_say_when_it_is_null() {
    // Arrange & Act
    let fields = FieldsGuide.fields();

    // Assert
    assert_eq!(
        fields
            .iter()
            .find(|field| field.field == "units[].to")
            .map(|field| field.means.as_str()),
        Some(
            "the version it takes its own crate to, a git unit's the short rev it takes its source to; null for the rest, for a compatible unit whose own move a pin holds -- `evidence.held` names that move, and the table writes `held` --, and for one whose crate does not move, only what it pulls in -- the table writes `stays`"
        )
    );
}

#[test]
fn fields_of_every_document_are_its_schema_command_and_diagnostics() {
    // Arrange & Act
    let fields = FieldsGuide.fields();

    // Assert
    assert_eq!(
        fields
            .iter()
            .filter(|field| field.document == "every")
            .map(|field| field.field.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "schema",
            "command",
            "diagnostics",
            "diagnostics[].severity",
            "diagnostics[].code",
            "diagnostics[].where",
            "diagnostics[].what",
            "diagnostics[].do",
            "diagnostics[].needs_person",
        ]
    );
}

#[test]
fn fields_of_the_footprint_say_what_the_parse_cannot_see() {
    // Arrange & Act
    let fields = FieldsGuide.fields();

    // Assert
    assert_eq!(
        fields
            .iter()
            .find(|field| field.field == "units[].evidence.footprint")
            .map(|field| field.means.as_str()),
        Some(
            "every `file:line` naming the crate, read from the sources with `syn`: a migration's work list. What a macro expands to is not seen, nor a function a glob import alone brings in; where a file could not be parsed, a `source-unparsed` warning says so, and the list is a floor"
        )
    );
}

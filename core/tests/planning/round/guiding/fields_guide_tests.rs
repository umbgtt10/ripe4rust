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

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::naming::declaration_collector::DeclarationCollector;
use cargo_ripe4rust::adapters::workspace::sources::naming::extern_declaration::ExternDeclaration;

#[test]
fn of_a_file_names_its_extern_crates_and_whether_they_bring_macros() {
    // Arrange
    let file = parsed("#[macro_use]\nextern crate log;\nextern crate itoa as numbers;\n");

    // Act
    let declarations = DeclarationCollector::of(&file);

    // Assert
    assert_eq!(
        declarations.externs,
        vec![
            ExternDeclaration {
                name: String::from("log"),
                alias: None,
                macro_use: true,
            },
            ExternDeclaration {
                name: String::from("itoa"),
                alias: Some(String::from("numbers")),
                macro_use: false,
            },
        ]
    );
}

#[test]
fn of_a_file_names_its_items_variants_and_parameters_at_every_depth() {
    // Arrange
    let file = parsed(
        "enum Kind {\n    Plain,\n}\nmod inner {\n    fn run<T, const N: usize>() {\n        struct Local;\n    }\n}\n",
    );

    // Act
    let declarations = DeclarationCollector::of(&file);

    // Assert
    assert_eq!(
        declarations.locals,
        ["Kind", "Plain", "inner", "run", "T", "N", "Local"].map(String::from)
    );
}

#[test]
fn of_a_file_names_its_macros_and_what_its_use_items_bring() {
    // Arrange
    let file = parsed(
        "use itoa::Buffer;\nfn run() {\n    use serde::Serialize;\n}\nmacro_rules! twice {\n    () => {};\n}\n",
    );

    // Act
    let declarations = DeclarationCollector::of(&file);

    // Assert
    assert_eq!(
        (
            declarations
                .uses
                .iter()
                .map(|leaf| leaf.bound.clone())
                .collect::<Vec<Option<String>>>(),
            declarations.macros
        ),
        (
            vec![
                Some(String::from("Buffer")),
                Some(String::from("Serialize"))
            ],
            vec![String::from("twice")]
        )
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::walking::declared_module::DeclaredModule;
use cargo_ripe4rust::adapters::workspace::sources::walking::module_declarations::ModuleDeclarations;
use syn::Item;
use syn::ItemMod;

fn module(
    name: &str,
    inline: &[&str],
    path: Option<&str>,
    public: bool,
    line: u32,
) -> DeclaredModule {
    DeclaredModule {
        name: String::from(name),
        inline: inline.iter().map(|name| String::from(*name)).collect(),
        path: path.map(String::from),
        public,
        line,
    }
}

#[test]
fn of_a_file_is_every_module_it_declares_out_of_line_through_inline_ones() {
    // Arrange
    let file = parsed(
        "pub mod a;\nmod b;\npub mod c {\n    pub mod d;\n    mod e;\n}\nmod f {\n    pub mod g;\n}\nmod h {}\n",
    );

    // Act
    let declared = ModuleDeclarations.of(&file.items, &[], true);

    // Assert
    assert_eq!(
        declared,
        vec![
            module("a", &[], None, true, 1),
            module("b", &[], None, false, 2),
            module("d", &["c"], None, true, 4),
            module("e", &["c"], None, false, 5),
            module("g", &["f"], None, false, 8),
        ]
    );
}

#[test]
fn of_a_module_not_public_declares_nothing_public() {
    // Arrange
    let file = parsed("pub mod a;\n");

    // Act
    let declared = ModuleDeclarations.of(&file.items, &[], false);

    // Assert
    assert_eq!(declared, vec![module("a", &[], None, false, 1)]);
}

#[test]
fn of_a_module_with_a_path_names_its_path_and_a_raw_name_its_file_s() {
    // Arrange
    let file = parsed("#[path = \"shared/io.rs\"]\nmod io;\nmod r#type;\n");

    // Act
    let declared = ModuleDeclarations.of(&file.items, &[], true);

    // Assert
    assert_eq!(
        declared,
        vec![
            module("io", &[], Some("shared/io.rs"), false, 2),
            module("type", &[], None, false, 3),
        ]
    );
}

#[test]
fn path_of_a_module_without_a_path_attribute_is_none() {
    // Arrange
    let file = parsed("#[cfg(test)]\nmod tests;\n#[path = \"io.rs\"]\nmod io;\n");
    let modules = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(module) => Some(module),
            _ => None,
        })
        .collect::<Vec<&ItemMod>>();

    // Act
    let paths = modules
        .iter()
        .map(|module| ModuleDeclarations.path(module))
        .collect::<Vec<Option<String>>>();

    // Assert
    assert_eq!(paths, vec![None, Some(String::from("io.rs"))]);
}

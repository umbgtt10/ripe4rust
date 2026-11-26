// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::app;
use crate::support::sources::dependency;
use crate::support::sources::member;
use crate::support::sources::named;
use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::declaration_collector::DeclarationCollector;
use cargo_ripe4rust::adapters::workspace::sources::naming::import_scope::ImportScope;
use cargo_ripe4rust::adapters::workspace::sources::naming::name_finder::NameFinder;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;
use std::slice::from_ref;
use syn::Item;
use syn::visit::Visit;

fn lines(found: &[&str]) -> Vec<String> {
    found.iter().map(|line| String::from(*line)).collect()
}

#[test]
fn found_of_a_dependency_s_own_name_alone_in_code_is_none() {
    // Arrange
    let text = "fn length(url: &str) -> usize {\n    url.len()\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, Vec::<String>::new());
}

#[test]
fn found_of_a_derive_names_its_crate_by_path_import_or_glob() {
    // Arrange
    let text = "use serde::Serialize;\n#[derive(Debug, Serialize)]\nstruct A;\n#[derive(serde::Deserialize)]\nstruct B;\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["serde:1", "serde:2", "serde:4"]));
}

#[test]
fn found_of_a_derive_through_a_glob_is_the_glob_s_crate() {
    // Arrange
    let text = "use serde::*;\n#[derive(Clone, Serialize)]\nstruct A;\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["serde:1", "serde:2"]));
}

#[test]
fn found_of_a_doc_comment_is_none() {
    // Arrange
    let text = "/// Formats with `itoa::Buffer`.\nfn digits() {}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, Vec::<String>::new());
}

#[test]
fn found_of_a_hyphenated_package_is_its_name_with_underscores() {
    // Arrange
    let text = "fn name() -> smol_str::SmolStr {\n    todo!()\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["smol-str:1"]));
}

#[test]
fn found_of_a_local_module_named_as_a_dependency_is_none() {
    // Arrange
    let text = "mod log {\n    pub fn info() {}\n}\nfn run() {\n    log::info();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, Vec::<String>::new());
}

#[test]
fn found_of_a_macro_by_path_and_by_macro_use_is_its_crate_s_lines() {
    // Arrange
    let text = "#[macro_use]\nextern crate log;\nfn run() {\n    info!(\"x\");\n    log::warn!(\"y\");\n    println!(\"z\");\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["log:2", "log:4", "log:5"]));
}

#[test]
fn found_of_a_macro_defined_in_the_target_is_none() {
    // Arrange
    let text = "#[macro_use]\nextern crate log;\nmacro_rules! info {\n    () => {};\n}\nfn run() {\n    info!();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["log:2"]));
}

#[test]
fn found_of_a_macro_s_arguments_names_what_they_hold() {
    // Arrange
    let text = "fn check() {\n    assert_eq!(itoa::Buffer::new().format(1), \"1\");\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:2"]));
}

#[test]
fn found_of_a_package_renamed_in_the_manifest_is_the_package() {
    // Arrange
    let renamed = member(vec![dependency("itoa", Some("numbers"))]);
    let text = "fn digits() {\n    numbers::Buffer::new();\n}\n";

    // Act
    let found = named(text, &renamed);

    // Assert
    assert_eq!(found, lines(&["itoa:2"]));
}

#[test]
fn found_of_a_path_from_the_root_is_its_crate() {
    // Arrange
    let text = "fn digits() {\n    ::itoa::Buffer::new();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:2"]));
}

#[test]
fn found_of_a_path_rooted_at_a_dependency_is_its_line() {
    // Arrange
    let text = "fn digits() {\n    let buffer = itoa::Buffer::new();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:2"]));
}

#[test]
fn found_of_a_rename_in_a_use_item_follows_the_new_name() {
    // Arrange
    let text = "use itoa as numbers;\nfn digits() {\n    numbers::Buffer::new();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:1", "itoa:3"]));
}

#[test]
fn found_of_a_use_item_and_the_names_it_binds_is_every_line_using_them() {
    // Arrange
    let text = "use itoa::Buffer;\n\nfn digits() -> Buffer {\n    Buffer::new()\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:1", "itoa:3", "itoa:4"]));
}

#[test]
fn found_of_an_attribute_the_crate_owns_is_its_crate() {
    // Arrange
    let text = "#[serde(rename_all = \"camelCase\")]\nstruct A;\n#[tokio::main]\nasync fn main() {}\n#[test]\nfn plain() {}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["serde:1", "tokio:3"]));
}

#[test]
fn found_of_an_extern_crate_renamed_follows_the_new_name() {
    // Arrange
    let text = "extern crate itoa as numbers;\nfn digits() {\n    numbers::Buffer::new();\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itoa:1", "itoa:3"]));
}

#[test]
fn found_of_names_used_through_a_glob_is_the_glob_s_crate() {
    // Arrange
    let text = "use itertools::*;\nfn pick() -> Either<u8, u8> {\n    Either::Left(1)\n}\nfn plain() -> std::fmt::Result {\n    Ok(())\n}\n";

    // Act
    let found = named(text, &app());

    // Assert
    assert_eq!(found, lines(&["itertools:1", "itertools:2", "itertools:3"]));
}

#[test]
fn packages_of_a_fragment_are_the_crates_it_names_once_each() {
    // Arrange
    let file =
        parsed("fn digits(a: itoa::Buffer, b: itoa::Buffer) -> url::Url {\n    todo!()\n}\n");
    let declarations = DeclarationCollector::of(&file);
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);
    let mut finder = NameFinder::new(&scope);
    let Some(Item::Fn(function)) = file.items.first() else {
        panic!("a function");
    };

    // Act
    finder.visit_signature(&function.sig);
    let packages = finder.packages();

    // Assert
    assert_eq!(packages, vec![String::from("itoa"), String::from("url")]);
}

#[test]
fn uses_of_a_use_item_are_its_root_s_line_and_each_leaf_s() {
    // Arrange
    let file = parsed("use serde::{\n    Deserialize,\n    Serialize,\n};\n");
    let Some(Item::Use(item)) = file.items.first() else {
        panic!("a use item");
    };

    // Act
    let found = scoped("", |scope| NameFinder::new(scope).uses(item));

    // Assert
    assert_eq!(
        found.iter().map(|named| named.line).collect::<Vec<u32>>(),
        vec![1, 2, 1, 3]
    );
}

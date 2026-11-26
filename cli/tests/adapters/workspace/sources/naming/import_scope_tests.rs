// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::app;
use crate::support::sources::declared;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::import_scope::ImportScope;
use cargo_ripe4rust::adapters::workspace::sources::naming::resolution::Resolution;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;
use std::slice::from_ref;

fn crate_named(name: &str) -> Resolution {
    Resolution::Crate(String::from(name))
}

fn resolved(text: &str, name: &str, global: bool, own_name: bool) -> Resolution {
    let declarations = declared(text);
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    ImportScope::new(&declarations, &codes, &macros).root(name, global, own_name)
}

#[test]
fn defines_macro_of_one_the_target_defines_is_true() {
    // Arrange
    let declarations = declared("macro_rules! twice {\n    () => {};\n}\n");
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);

    // Act
    let defined = [scope.defines_macro("twice"), scope.defines_macro("info")];

    // Assert
    assert_eq!(defined, [true, false]);
}

#[test]
fn globs_are_the_dependencies_a_file_glob_imports_through_any_binding() {
    // Arrange
    let declarations = declared(
        "use itertools::*;\nuse serde::de;\nuse de::*;\nuse super::*;\nuse std::fmt::*;\n",
    );
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);

    // Act
    let scope = ImportScope::new(&declarations, &codes, &macros);

    // Assert
    assert_eq!(
        scope.globs(),
        [String::from("itertools"), String::from("serde")]
    );
}

#[test]
fn macro_crates_are_the_globs_and_the_macro_use_crates() {
    // Arrange
    let declarations = declared("#[macro_use]\nextern crate log;\nuse itertools::*;\n");
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);

    // Act
    let crates = scope.macro_crates();

    // Assert
    assert_eq!(crates, vec![String::from("itertools"), String::from("log")]);
}

#[test]
fn root_of_a_binding_through_another_binding_is_its_crate() {
    // Arrange & Act
    let resolution = resolved(
        "use Error as Failure;\nuse de::Error;\nuse serde::de;\n",
        "Failure",
        false,
        false,
    );

    // Assert
    assert_eq!(resolution, crate_named("serde"));
}

#[test]
fn root_of_a_dependency_s_name_alone_is_local() {
    // Arrange & Act
    let resolution = resolved("", "url", false, false);

    // Assert
    assert_eq!(resolution, Resolution::Local);
}

#[test]
fn root_of_a_dependency_s_name_starting_a_path_is_its_crate() {
    // Arrange & Act
    let resolution = resolved("", "url", false, true);

    // Assert
    assert_eq!(resolution, crate_named("url"));
}

#[test]
fn root_of_a_global_path_is_a_dependency_or_local() {
    // Arrange & Act
    let resolutions = [
        resolved("mod itoa {}\n", "itoa", true, true),
        resolved("", "std", true, true),
    ];

    // Assert
    assert_eq!(resolutions, [crate_named("itoa"), Resolution::Local]);
}

#[test]
fn root_of_a_keyword_is_local() {
    // Arrange & Act
    let resolutions =
        ["Self", "crate", "self", "super"].map(|name| resolved("", name, false, true));

    // Assert
    assert_eq!(resolutions, [const { Resolution::Local }; 4]);
}

#[test]
fn root_of_a_name_bound_to_the_workspace_s_own_code_is_local() {
    // Arrange & Act
    let resolution = resolved("use crate::numbers::Buffer;\n", "Buffer", false, false);

    // Assert
    assert_eq!(resolution, Resolution::Local);
}

#[test]
fn root_of_a_name_nothing_declares_is_unknown() {
    // Arrange & Act
    let resolution = resolved("", "Either", false, false);

    // Assert
    assert_eq!(resolution, Resolution::Unknown);
}

#[test]
fn root_of_an_extern_crate_alias_is_its_crate() {
    // Arrange & Act
    let resolution = resolved("extern crate itoa as numbers;\n", "numbers", false, false);

    // Assert
    assert_eq!(resolution, crate_named("itoa"));
}

#[test]
fn root_of_an_item_or_a_prelude_name_is_local() {
    // Arrange & Act
    let resolutions = [
        resolved("struct Either;\n", "Either", false, false),
        resolved("mod url {}\n", "url", false, true),
        resolved("", "Option", false, false),
    ];

    // Assert
    assert_eq!(resolutions, [const { Resolution::Local }; 3]);
}

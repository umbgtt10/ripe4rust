// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::app;
use crate::support::sources::declared;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;

#[test]
fn defines_of_a_macro_any_file_of_the_target_defines_is_true() {
    // Arrange
    let files = [
        declared("macro_rules! twice {\n    () => {};\n}\n"),
        declared("fn run() {}\n"),
    ];
    let member = app();

    // Act
    let macros = TargetMacros::new(&files, &CodeNames::new(&member));

    // Assert
    assert_eq!(
        [macros.defines("twice"), macros.defines("run")],
        [true, false]
    );
}

#[test]
fn macro_use_is_every_dependency_a_macro_use_extern_crate_brings_once() {
    // Arrange
    let files = [
        declared("#[macro_use]\nextern crate log;\nextern crate itoa;\n"),
        declared("#[macro_use]\nextern crate log;\n#[macro_use]\nextern crate core;\n"),
    ];
    let member = app();

    // Act
    let macros = TargetMacros::new(&files, &CodeNames::new(&member));

    // Assert
    assert_eq!(macros.macro_use(), [String::from("log")]);
}

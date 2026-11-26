// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::app;
use crate::support::sources::declared;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::import_scope::ImportScope;
use cargo_ripe4rust::adapters::workspace::sources::naming::named::Named;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;
use cargo_ripe4rust::adapters::workspace::sources::naming::token_names::TokenNames;
use proc_macro2::TokenStream;
use std::slice::from_ref;
use std::str::FromStr;

fn found(imports: &str, tokens: &str) -> Vec<Named> {
    let declarations = declared(imports);
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);
    TokenNames::new(&scope).of(TokenStream::from_str(tokens).expect("tokens"))
}

fn named(package: &str, line: u32) -> Named {
    Named {
        package: String::from(package),
        line,
    }
}

#[test]
fn of_a_name_a_use_item_binds_is_its_crate_alone_or_starting_a_path() {
    // Arrange & Act
    let names = found("use itoa::Buffer;\n", "Buffer, (\nBuffer::new())");

    // Assert
    assert_eq!(names, vec![named("itoa", 1), named("itoa", 2)]);
}

#[test]
fn of_a_name_continuing_something_else_is_none() {
    // Arrange & Act
    let names = found("", "value.itoa::x, $url::y, a::log::z");

    // Assert
    assert_eq!(names, Vec::new());
}

#[test]
fn of_a_path_rooted_at_a_dependency_in_brackets_is_its_crate() {
    // Arrange & Act
    let names = found("", "x: [\n(url::Url, log)]");

    // Assert
    assert_eq!(names, vec![named("url", 2)]);
}

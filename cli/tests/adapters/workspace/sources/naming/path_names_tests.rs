// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::lines;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::naming::path_names::PathNames;
use syn::Ident;
use syn::Path;
use syn::parse_str;

fn path(text: &str) -> Path {
    parse_str(text).expect("a path")
}

#[test]
fn ident_of_a_dependency_s_own_name_alone_is_none() {
    // Arrange
    let ident = parse_str::<Ident>("url").expect("an identifier");

    // Act
    let found = scoped("", |scope| {
        PathNames::new(scope).ident(&ident, false, false, &[])
    });

    // Assert
    assert_eq!(lines(&found), Vec::<String>::new());
}

#[test]
fn of_a_lowercase_name_alone_is_never_a_glob_s() {
    // Arrange & Act
    let found = scoped("use itertools::*;\n", |scope| {
        PathNames::new(scope).of(&path("value"))
    });

    // Assert
    assert_eq!(lines(&found), Vec::<String>::new());
}

#[test]
fn of_a_name_a_glob_may_have_brought_is_every_glob_s_crate() {
    // Arrange & Act
    let found = scoped("use itertools::*;\nuse serde::*;\n", |scope| {
        PathNames::new(scope).of(&path("Either"))
    });

    // Assert
    assert_eq!(lines(&found), ["itertools:1", "serde:1"]);
}

#[test]
fn of_a_path_from_the_root_is_its_crate() {
    // Arrange & Act
    let found = scoped("", |scope| {
        PathNames::new(scope).of(&path("::itoa::Buffer"))
    });

    // Assert
    assert_eq!(lines(&found), ["itoa:1"]);
}

#[test]
fn of_a_path_rooted_at_a_dependency_is_its_crate_at_its_line() {
    // Arrange & Act
    let found = scoped("", |scope| PathNames::new(scope).of(&path("itoa::Buffer")));

    // Assert
    assert_eq!(lines(&found), ["itoa:1"]);
}

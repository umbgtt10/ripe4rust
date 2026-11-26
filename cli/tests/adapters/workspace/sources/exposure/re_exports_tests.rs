// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::exposure::re_exports::ReExports;
use syn::Item;
use syn::ItemUse;

fn names(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| String::from(*name)).collect()
}

fn used(text: &str) -> ItemUse {
    let file = parsed(text);
    let Some(Item::Use(item)) = file.items.into_iter().next() else {
        panic!("a use item");
    };
    item
}

#[test]
fn modules_of_a_use_are_those_its_globs_open() {
    // Arrange
    let item = used("pub use a::{b::*, C, d::{e::*, F}, *};\n");

    // Act
    let modules = ReExports.modules(&item);

    // Assert
    assert_eq!(modules, names(&["b", "e", "a"]));
}

#[test]
fn names_of_a_use_are_the_items_its_names_stand_for() {
    // Arrange
    let item = used("pub use a::{B, c::D as E, f::{self}, g::{self as h}, i::*, J as _};\n");

    // Act
    let found = ReExports.names(&item);

    // Assert
    assert_eq!(found, names(&["B", "D", "f", "g", "J"]));
}

#[test]
fn names_of_a_use_of_one_name_alone_are_that_name() {
    // Arrange
    let item = used("pub use Thing;\n");

    // Act
    let found = ReExports.names(&item);

    // Assert
    assert_eq!(found, names(&["Thing"]));
}

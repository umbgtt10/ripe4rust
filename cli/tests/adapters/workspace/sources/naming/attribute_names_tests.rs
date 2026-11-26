// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::lines;
use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::naming::attribute_names::AttributeNames;
use syn::Attribute;
use syn::Item;

fn attribute(text: &str) -> Attribute {
    let file = parsed(&format!("{text}\nstruct A;\n"));
    let Some(Item::Struct(item)) = file.items.first() else {
        panic!("a struct");
    };
    item.attrs[0].clone()
}

#[test]
fn derives_of_a_derive_written_as_a_value_are_none() {
    // Arrange & Act
    let found = scoped("", |scope| {
        AttributeNames::new(scope).derives(&attribute("#[derive = \"Serialize\"]"))
    });

    // Assert
    assert_eq!(lines(&found), Vec::<String>::new());
}

#[test]
fn of_a_built_in_attribute_is_the_tokens_names_alone_globs_aside() {
    // Arrange & Act
    let found = scoped("use itertools::*;\n", |scope| {
        AttributeNames::new(scope).of(&attribute("#[cfg_attr(test, derive(serde::Serialize))]"))
    });

    // Assert
    assert_eq!(lines(&found), ["serde:1"]);
}

#[test]
fn of_a_derive_is_each_derived_crate_built_ins_aside() {
    // Arrange & Act
    let found = scoped("use serde::Deserialize;\nuse itertools::*;\n", |scope| {
        AttributeNames::new(scope).of(&attribute(
            "#[derive(Debug, serde::Serialize, Deserialize, Display)]",
        ))
    });

    // Assert
    assert_eq!(lines(&found), ["itertools:1", "serde:1", "serde:1"]);
}

#[test]
fn of_a_doc_comment_is_none() {
    // Arrange & Act
    let found = scoped("", |scope| {
        AttributeNames::new(scope).of(&attribute("/// Uses `itoa::Buffer`."))
    });

    // Assert
    assert_eq!(lines(&found), Vec::<String>::new());
}

#[test]
fn of_an_attribute_a_crate_owns_is_its_crate() {
    // Arrange & Act
    let found = scoped("", |scope| {
        AttributeNames::new(scope).of(&attribute("#[serde(rename = \"a\")]"))
    });

    // Assert
    assert_eq!(lines(&found), ["serde:1"]);
}

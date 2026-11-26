// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::module_items::ModuleItems;
use cargo_ripe4rust::adapters::workspace::sources::exposure::re_export_reach::ReExportReach;
use cargo_ripe4rust::adapters::workspace::sources::naming::item_name::ItemName;
use syn::File;

fn expected(items: &[(&str, bool)]) -> Vec<(String, bool)> {
    items
        .iter()
        .map(|(name, public)| (String::from(*name), *public))
        .collect()
}

fn reached(files: &[(&str, bool, Option<&str>)]) -> Vec<(String, bool)> {
    let syntax = files
        .iter()
        .map(|(text, _, _)| parsed(text))
        .collect::<Vec<File>>();
    scoped("", |scope| {
        let items = syntax
            .iter()
            .zip(files)
            .flat_map(|(file, (_, public, module))| {
                ModuleItems::items(&file.items, *public, *module, scope)
            })
            .collect();
        ReExportReach::default()
            .of(items)
            .into_iter()
            .filter_map(|placed| {
                ItemName
                    .of(placed.item)
                    .map(|ident| (ident.to_string(), placed.public))
            })
            .collect()
    })
}

#[test]
fn of_a_module_a_public_use_names_reaches_what_it_holds() {
    // Arrange & Act
    let items = reached(&[(
        "mod inner {\n    pub mod api {\n        pub fn f() {}\n    }\n    pub fn g() {}\n}\npub use inner::api;\n",
        true,
        None,
    )]);

    // Assert
    assert_eq!(
        items,
        expected(&[("inner", true), ("api", true), ("f", true), ("g", false)])
    );
}

#[test]
fn of_items_a_public_glob_opens_reaches_its_module_and_the_public_modules_in_it() {
    // Arrange & Act
    let items = reached(&[
        ("mod inner;\npub use inner::*;\n", true, None),
        (
            "pub struct A;\nmod deeper {\n    pub struct B;\n}\npub mod open {\n    pub struct C;\n}\n",
            false,
            Some("inner"),
        ),
    ]);

    // Assert
    assert_eq!(
        items,
        expected(&[
            ("inner", true),
            ("A", true),
            ("deeper", true),
            ("B", false),
            ("open", true),
            ("C", true),
        ])
    );
}

#[test]
fn of_items_a_public_use_names_reaches_them_where_they_are_defined() {
    // Arrange & Act
    let items = reached(&[
        ("mod inner;\npub use inner::Thing;\n", true, None),
        (
            "pub struct Thing;\npub struct Other;\n",
            false,
            Some("inner"),
        ),
    ]);

    // Assert
    assert_eq!(
        items,
        expected(&[("inner", true), ("Thing", true), ("Other", false)])
    );
}

#[test]
fn of_items_a_use_short_of_pub_names_reaches_none() {
    // Arrange & Act
    let items = reached(&[
        (
            "mod inner;\nuse inner::Thing;\npub(crate) use inner::Other;\n",
            true,
            None,
        ),
        (
            "pub struct Thing;\npub struct Other;\n",
            false,
            Some("inner"),
        ),
    ]);

    // Assert
    assert_eq!(
        items,
        expected(&[("inner", true), ("Thing", false), ("Other", false)])
    );
}

#[test]
fn of_items_re_exported_again_from_a_private_module_reaches_the_first() {
    // Arrange & Act
    let items = reached(&[
        ("mod a;\nmod b;\npub use a::Y;\n", true, None),
        ("pub use crate::b::X as Y;\n", false, Some("a")),
        ("pub struct X;\npub struct Z;\n", false, Some("b")),
    ]);

    // Assert
    assert_eq!(
        items,
        expected(&[("a", true), ("b", true), ("X", true), ("Z", false)])
    );
}

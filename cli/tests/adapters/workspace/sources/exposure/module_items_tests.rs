// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::module_items::ModuleItems;
use cargo_ripe4rust::adapters::workspace::sources::exposure::placed_item::PlacedItem;
use cargo_ripe4rust::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use cargo_ripe4rust::adapters::workspace::sources::naming::item_name::ItemName;
use cargo_ripe4rust::adapters::workspace::sources::walking::parsed_file::ParsedFile;
use std::path::PathBuf;

fn placed(items: &[PlacedItem<'_>]) -> Vec<(String, bool)> {
    items
        .iter()
        .filter_map(|placed| {
            ItemName
                .of(placed.item)
                .map(|ident| (ident.to_string(), placed.public))
        })
        .collect()
}

#[test]
fn items_of_inline_modules_are_public_where_every_module_on_the_way_is() {
    // Arrange
    let file = parsed(
        "pub fn a() {}\npub mod b {\n    pub fn c() {}\n    mod d {\n        pub fn e() {}\n    }\n}\nmod f {\n    pub mod g {\n        pub fn h() {}\n    }\n}\n",
    );

    // Act
    let items = scoped("", |scope| {
        placed(&ModuleItems::items(&file.items, true, scope))
    });

    // Assert
    assert_eq!(
        items,
        [
            ("a", true),
            ("b", true),
            ("c", true),
            ("d", true),
            ("e", false),
            ("f", true),
            ("g", false),
            ("h", false),
        ]
        .map(|(name, public)| (String::from(name), public))
    );
}

#[test]
fn of_files_is_every_item_of_each_public_as_its_file() {
    // Arrange
    let files =
        [("pub fn a() {}\n", true), ("pub fn b() {}\n", false)].map(|(text, public)| ParsedFile {
            path: PathBuf::from("/work/ws/app/src/lib.rs"),
            name: String::from("app/src/lib.rs"),
            syntax: parsed(text),
            public,
        });

    // Act
    let items = scoped("", |scope| {
        let scoped_files = files
            .iter()
            .map(|file| ScopedFile { file, scope })
            .collect::<Vec<ScopedFile<'_>>>();
        placed(&ModuleItems.of(&scoped_files))
    });

    // Assert
    assert_eq!(
        items,
        [("a", true), ("b", false)].map(|(name, public)| (String::from(name), public))
    );
}

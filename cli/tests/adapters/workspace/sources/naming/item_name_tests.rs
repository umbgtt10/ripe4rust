// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::naming::item_name::ItemName;

#[test]
fn of_every_kind_of_named_item_is_its_name_and_of_the_rest_none() {
    // Arrange
    let file = parsed(
        "const A: u8 = 1;\nenum B {}\nfn c() {}\nmod d {}\nstatic E: u8 = 1;\nstruct F;\ntrait G {}\ntrait H = G;\ntype I = u8;\nunion J { a: u8 }\nuse std::fmt;\nimpl F {}\n",
    );

    // Act
    let names = file
        .items
        .iter()
        .map(|item| ItemName.of(item).map(ToString::to_string))
        .collect::<Vec<Option<String>>>();

    // Assert
    assert_eq!(
        names,
        ["A", "B", "c", "d", "E", "F", "G", "H", "I", "J"]
            .map(|name| Some(String::from(name)))
            .into_iter()
            .chain([None, None])
            .collect::<Vec<Option<String>>>()
    );
}

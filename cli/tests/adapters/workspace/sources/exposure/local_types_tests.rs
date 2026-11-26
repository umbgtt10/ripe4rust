// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::local_types::LocalTypes;
use cargo_ripe4rust::adapters::workspace::sources::exposure::module_items::ModuleItems;
use syn::Path;
use syn::Type;
use syn::parse_str;

const LIBRARY: &str = "pub struct Shown;\nstruct Hidden;\npub enum Kind {}\npub type Alias = u8;\npub union Both { a: u8 }\npub trait Open {}\ntrait Closed {}\nmod inner {\n    pub struct Inner;\n}\nstruct Twice;\npub mod outer {\n    pub struct Twice;\n}\n";

fn answers(run: impl Fn(&LocalTypes) -> Vec<bool>) -> Vec<bool> {
    let file = parsed(LIBRARY);
    scoped("", |scope| {
        let items = ModuleItems::items(&file.items, true, scope);
        run(&LocalTypes::new(&items))
    })
}

fn ty(text: &str) -> Type {
    parse_str(text).expect("a type")
}

#[test]
fn public_type_is_a_type_defined_pub_in_a_public_module() {
    // Arrange & Act
    let public = answers(|types| {
        [
            "Shown", "&Shown", "Kind", "Alias", "Both", "Hidden", "Inner", "Twice", "Vec<u8>",
            "(u8, u8)",
        ]
        .map(|text| types.public_type(&ty(text)))
        .to_vec()
    });

    // Assert
    assert_eq!(
        public,
        [
            true, true, true, true, true, false, false, true, false, false
        ]
    );
}

#[test]
fn reachable_is_a_public_type_or_one_the_library_does_not_define() {
    // Arrange & Act
    let reachable = answers(|types| {
        ["Shown", "Hidden", "Inner", "itoa::Buffer", "[u8]"]
            .map(|text| types.reachable(&ty(text)))
            .to_vec()
    });

    // Assert
    assert_eq!(reachable, [true, false, false, true, true]);
}

#[test]
fn reachable_trait_is_a_public_trait_or_one_the_library_does_not_define() {
    // Arrange & Act
    let reachable = answers(|types| {
        ["Open", "Closed", "serde::Serialize", "crate::Closed"]
            .map(|text| types.reachable_trait(&parse_str::<Path>(text).expect("a path")))
            .to_vec()
    });

    // Assert
    assert_eq!(reachable, [true, false, true, false]);
}

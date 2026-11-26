// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::naming::use_leaf::UseLeaf;
use cargo_ripe4rust::adapters::workspace::sources::naming::use_leaves::UseLeaves;
use syn::Item;

fn leaf(root: &str, bound: Option<&str>, line: u32, glob: bool) -> UseLeaf {
    UseLeaf {
        root: String::from(root),
        root_line: 1,
        global: false,
        bound: bound.map(String::from),
        line,
        glob,
    }
}

fn leaves(text: &str) -> Vec<UseLeaf> {
    let file = parsed(text);
    let Some(Item::Use(item)) = file.items.first() else {
        panic!("a use item");
    };
    UseLeaves::new(item).of(&item.tree)
}

#[test]
fn of_a_crate_alone_binds_its_own_name() {
    // Arrange & Act
    let found = leaves("use serde;\n");

    // Assert
    assert_eq!(found, vec![leaf("serde", Some("serde"), 1, false)]);
}

#[test]
fn of_a_global_path_says_so() {
    // Arrange & Act
    let found = leaves("use ::itoa::Buffer;\n");

    // Assert
    assert_eq!(
        found,
        vec![UseLeaf {
            global: true,
            ..leaf("itoa", Some("Buffer"), 1, false)
        }]
    );
}

#[test]
fn of_a_group_is_every_name_it_brings_rooted_at_the_first() {
    // Arrange & Act
    let found = leaves(
        "use serde::{\n    Deserialize,\n    de::{self, Error as Failure},\n    ser::*,\n    Serialize as _,\n};\n",
    );

    // Assert
    assert_eq!(
        found,
        vec![
            leaf("serde", Some("Deserialize"), 2, false),
            leaf("serde", Some("de"), 3, false),
            leaf("serde", Some("Failure"), 3, false),
            leaf("serde", None, 4, true),
            leaf("serde", None, 5, false),
        ]
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::policy_section::PolicySection;
use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::values::location::Location;

fn at(line: u32) -> Location {
    Location {
        file: String::from("Cargo.toml"),
        line,
    }
}

#[test]
fn tree_of_a_package_s_own_metadata_is_not_the_workspace_s() {
    // Arrange & Act
    let tree = PolicySection.tree("[package.metadata.ripe4rust]\ngates = [\"just stage1\"]\n");

    // Assert
    assert_eq!(
        tree,
        Ok(PolicyTree {
            at: None,
            entries: Vec::new(),
        })
    );
}

#[test]
fn tree_of_a_section_written_with_dotted_keys_is_read_the_same() {
    // Arrange & Act
    let tree = PolicySection.tree("[workspace.metadata]\nripe4rust.gates = [\"just stage1\"]\n");

    // Assert
    assert_eq!(
        tree,
        Ok(PolicyTree {
            at: Some(at(2)),
            entries: vec![PolicyEntry {
                key: String::from("gates"),
                value: PolicyValue::List(vec![PolicyValue::Text(String::from("just stage1"))]),
                at: at(2),
            }],
        })
    );
}

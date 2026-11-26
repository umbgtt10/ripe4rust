// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::layout_reader::LayoutReader;
use cargo_ripe4rust::system::model::layout::Layout;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

#[test]
fn layout_of_output_that_is_not_metadata_is_internal() {
    // Arrange & Act
    let layout = LayoutReader.layout("[]");

    // Assert
    assert!(
        matches!(&layout, Err(PortFailure::Internal { doing, .. }) if doing == "reading what `cargo metadata --no-deps` printed"),
        "{layout:?}"
    );
}

#[test]
fn layout_of_the_recorded_workspace_is_its_root_and_its_members_manifests() {
    // Arrange & Act
    let layout = LayoutReader.layout(include_str!(
        "../../fixtures/workspace/cargo/metadata-no-deps.json"
    ));

    // Assert
    assert_eq!(
        layout,
        Ok(Layout {
            root: PathBuf::from("/work/ws"),
            members: vec![
                (
                    String::from("app"),
                    PathBuf::from("/work/ws/app/Cargo.toml")
                ),
                (
                    String::from("app-core"),
                    PathBuf::from("/work/ws/app-core/Cargo.toml")
                ),
            ],
        })
    );
}

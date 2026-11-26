// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::layout_reader::LayoutReader;
use cargo_ripe4rust::system::model::declared_dependency::DeclaredDependency;
use cargo_ripe4rust::system::model::layout::Layout;
use cargo_ripe4rust::system::model::member_sources::MemberSources;
use cargo_ripe4rust::system::model::source_target::SourceTarget;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

fn declared(name: &str, source: Option<&str>) -> DeclaredDependency {
    DeclaredDependency {
        name: String::from(name),
        rename: None,
        source: source.map(String::from),
    }
}

fn target(kind: &str, root: &str) -> SourceTarget {
    SourceTarget {
        kinds: vec![String::from(kind)],
        root: PathBuf::from(root),
    }
}

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
fn layout_of_the_recorded_workspace_is_its_root_and_its_members_manifests_and_sources() {
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
            sources: vec![
                MemberSources {
                    name: String::from("app"),
                    targets: vec![target("bin", "/work/ws/app/src/main.rs")],
                    dependencies: vec![
                        declared("app-core", None),
                        declared("itertools", Some(CRATES_IO)),
                        declared("smallvec", Some(CRATES_IO)),
                        declared("memchr", Some(CRATES_IO)),
                    ],
                },
                MemberSources {
                    name: String::from("app-core"),
                    targets: vec![target("lib", "/work/ws/app-core/src/lib.rs")],
                    dependencies: vec![declared("itoa", Some(CRATES_IO))],
                },
            ],
        })
    );
}

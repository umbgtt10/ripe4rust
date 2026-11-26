// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::workspace::cargo_layout::CargoLayout;
use cargo_ripe4rust::adapters::workspace::manifest_files::ManifestFiles;
use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::traits::workspace_files::WorkspaceFiles;
use ripe4rust_core::values::location::Location;
use std::path::PathBuf;

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

fn dir() -> PathBuf {
    PathBuf::from("/work/ws/app")
}

fn entry(key: &str, value: PolicyValue, line: u32) -> PolicyEntry {
    PolicyEntry {
        key: String::from(key),
        value,
        at: at("Cargo.toml", line),
    }
}

fn texts(items: &[&str]) -> PolicyValue {
    PolicyValue::List(
        items
            .iter()
            .map(|item| PolicyValue::Text(String::from(*item)))
            .collect(),
    )
}

#[test]
fn fingerprint_of_an_unreadable_manifest_names_it() {
    // Arrange
    let (runner, files) = (layout(), files().refusing("/work/ws/app/Cargo.toml"));

    // Act
    let fingerprint = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).fingerprint();

    // Assert
    assert_eq!(
        fingerprint,
        Err(PortFailure::WorkspaceUnreadable {
            file: String::from("app/Cargo.toml"),
            detail: String::from("permission denied"),
        })
    );
}

#[test]
fn fingerprint_of_the_recorded_workspace_changes_with_its_lock() {
    // Arrange
    let (runner, before) = (layout(), files());
    let after = files().with("/work/ws/Cargo.lock", "version = 4\n");

    // Act
    let fingerprints = [
        ManifestFiles::new(&CargoLayout::new(&runner, dir()), &before).fingerprint(),
        ManifestFiles::new(&CargoLayout::new(&runner, dir()), &after).fingerprint(),
    ];

    // Assert
    assert_eq!(fingerprints[0].as_ref().map(String::len), Ok(16));
    assert_ne!(fingerprints[0], fingerprints[1]);
}

#[test]
fn policy_of_a_manifest_without_one_is_empty() {
    // Arrange
    let runner = layout();
    let files = files().with(
        "/work/ws/Cargo.toml",
        "[workspace]\nmembers = [\"app\", \"app-core\"]\n",
    );

    // Act
    let policy = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).policy();

    // Assert
    assert_eq!(
        policy,
        Ok(PolicyTree {
            at: None,
            entries: Vec::new(),
        })
    );
}

#[test]
fn policy_of_a_value_that_is_no_table_is_unreadable() {
    // Arrange
    let runner = layout();
    let files = files().with(
        "/work/ws/Cargo.toml",
        "[workspace.metadata]\nripe4rust = 3\n",
    );

    // Act
    let policy = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).policy();

    // Assert
    assert_eq!(
        policy,
        Err(PortFailure::WorkspaceUnreadable {
            file: String::from("Cargo.toml"),
            detail: String::from("line 2: `[workspace.metadata.ripe4rust]` is not a table"),
        })
    );
}

#[test]
fn policy_of_the_recorded_workspace_is_its_table_line_by_line() {
    // Arrange
    let (runner, files) = (layout(), files());

    // Act
    let policy = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).policy();

    // Assert
    let pin = PolicyValue::Table(vec![
        PolicyEntry {
            key: String::from("reason"),
            value: PolicyValue::Text(String::from("the iterator chains are rewritten for 0.14")),
            at: at("Cargo.toml", 20),
        },
        PolicyEntry {
            key: String::from("review"),
            value: PolicyValue::Text(String::from("2027-03-01")),
            at: at("Cargo.toml", 20),
        },
    ]);
    assert_eq!(
        policy,
        Ok(PolicyTree {
            at: Some(at("Cargo.toml", 14)),
            entries: vec![
                entry("gates", texts(&["cargo test"]), 15),
                entry(
                    "pins",
                    PolicyValue::Table(vec![entry("itertools", pin, 20)]),
                    19
                ),
                entry("published", texts(&["app-core"]), 16),
                entry(
                    "validate-beyond-gates",
                    texts(&["thumbv7em-none-eabihf"]),
                    17
                ),
            ],
        })
    );
}

#[test]
fn policy_reads_the_layout_once_from_cargo_wherever_it_runs() {
    // Arrange
    let (runner, files) = (layout(), files());
    let source = CargoLayout::new(&runner, dir());
    let manifests = ManifestFiles::new(&source, &files);

    // Act
    let _ = (
        manifests.policy(),
        manifests.toolchain(),
        manifests.fingerprint(),
    );

    // Assert
    assert_eq!(
        runner.commands(),
        vec![String::from("cargo metadata --format-version 1 --no-deps")]
    );
    assert_eq!(runner.calls.borrow()[0].dir, dir());
}

#[test]
fn requirement_lines_of_the_recorded_crates_are_where_each_is_stated() {
    // Arrange
    let (runner, files) = (layout(), files());
    let source = CargoLayout::new(&runner, dir());
    let manifests = ManifestFiles::new(&source, &files);

    // Act
    let lines = ["itertools", "itoa", "memchr", "smallvec", "either"]
        .map(|name| manifests.requirement_lines(name));

    // Assert
    assert_eq!(
        lines,
        [
            Ok(vec![at("Cargo.toml", 12)]),
            Ok(vec![at("app-core/Cargo.toml", 9)]),
            Ok(vec![at("app/Cargo.toml", 15)]),
            Ok(vec![at("app/Cargo.toml", 12)]),
            Ok(Vec::new()),
        ]
    );
}

#[test]
fn rust_version_line_of_each_recorded_member_is_where_it_is_declared() {
    // Arrange
    let (runner, files) = (layout(), files());
    let source = CargoLayout::new(&runner, dir());
    let manifests = ManifestFiles::new(&source, &files);

    // Act
    let lines = ["app", "app-core", "absent"].map(|member| manifests.rust_version_line(member));

    // Assert
    assert_eq!(
        lines,
        [
            Ok(Some(at("Cargo.toml", 8))),
            Ok(Some(at("app-core/Cargo.toml", 5))),
            Ok(None),
        ]
    );
}

#[test]
fn toolchain_of_a_workspace_without_a_toolchain_file_is_none() {
    // Arrange
    let runner = layout();
    let files = MemoryFiles::new().with(
        "/work/ws/Cargo.toml",
        include_str!("../../fixtures/workspace/files/Cargo.toml.txt"),
    );

    // Act
    let toolchain = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).toolchain();

    // Assert
    assert_eq!(toolchain, Ok(None));
}

#[test]
fn toolchain_of_the_recorded_workspace_is_its_channel() {
    // Arrange
    let (runner, files) = (layout(), files());

    // Act
    let toolchain = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).toolchain();

    // Assert
    assert_eq!(
        toolchain,
        Ok(Some(ToolchainFile {
            channel: String::from("1.99.1"),
            at: at("rust-toolchain.toml", 2),
        }))
    );
}

#[test]
fn written_of_a_workspace_without_a_toolchain_file_names_no_channel() {
    // Arrange
    let runner = layout();
    let files = files();
    files
        .files
        .borrow_mut()
        .remove("/work/ws/rust-toolchain.toml");

    // Act
    let lines = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).written();

    // Assert
    assert_eq!(
        lines.map(|lines| lines.iter().any(|line| line.key == "channel")),
        Ok(false)
    );
}

#[test]
fn written_of_the_recorded_workspace_is_every_line_a_unit_can_edit() {
    // Arrange
    let (runner, files) = (layout(), files());
    let written = |file: &str, line: u32, key: &str, value: &str| WrittenLine {
        at: at(file, line),
        key: String::from(key),
        value: String::from(value),
    };

    // Act
    let lines = ManifestFiles::new(&CargoLayout::new(&runner, dir()), &files).written();

    // Assert
    assert_eq!(
        lines,
        Ok(vec![
            written("Cargo.toml", 8, "rust-version", "1.85"),
            written("Cargo.toml", 12, "itertools", "0.10"),
            written("app-core/Cargo.toml", 5, "rust-version", "1.85"),
            written("app-core/Cargo.toml", 9, "itoa", "0.4"),
            written("app/Cargo.toml", 12, "smallvec", "1.6.0"),
            written("app/Cargo.toml", 15, "memchr", "2.5"),
            written("rust-toolchain.toml", 2, "channel", "1.99.1"),
        ])
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::lock_reader::LockReader;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;

fn entry(name: &str, version: &str) -> LockEntry {
    LockEntry {
        name: String::from(name),
        version: Version::parse(version).expect("a version"),
    }
}

#[test]
fn entries_of_a_lock_that_does_not_parse_are_unreadable() {
    // Arrange & Act
    let entries = LockReader.entries("[[package]\nname = \"itoa\"\n");

    // Assert
    assert!(
        matches!(&entries, Err(PortFailure::WorkspaceUnreadable { file, .. }) if file == "Cargo.lock"),
        "{entries:?}"
    );
}

#[test]
fn entries_of_a_package_with_no_version_are_unreadable() {
    // Arrange & Act
    let entries = LockReader.entries(
        "version = 4\n\n[[package]]\nname = \"itoa\"\nversion = \"four\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );

    // Assert
    assert_eq!(
        entries,
        Err(PortFailure::WorkspaceUnreadable {
            file: String::from("Cargo.lock"),
            detail: String::from("the package `itoa` is at `four`, which is not a version"),
        })
    );
}

#[test]
fn entries_of_the_recorded_lock_are_its_crates_io_packages_in_order() {
    // Arrange & Act
    let entries = LockReader.entries(include_str!(
        "../../fixtures/workspace/files/Cargo.lock.txt"
    ));

    // Assert
    assert_eq!(
        entries,
        Ok(vec![
            entry("either", "1.5.0"),
            entry("itertools", "0.10.0"),
            entry("itoa", "0.4.7"),
            entry("memchr", "2.5.0"),
            entry("smallvec", "1.6.0"),
        ])
    );
}

#[test]
fn entries_of_the_sparse_registry_are_crates_io_s_too() {
    // Arrange & Act
    let entries = LockReader.entries(
        "version = 4\n\n[[package]]\nname = \"zeta\"\nversion = \"1.0.0\"\nsource = \"sparse+https://index.crates.io/\"\n\n[[package]]\nname = \"alpha\"\nversion = \"2.0.0\"\nsource = \"git+https://example.org/alpha#0abc\"\n",
    );

    // Assert
    assert_eq!(entries, Ok(vec![entry("zeta", "1.0.0")]));
}

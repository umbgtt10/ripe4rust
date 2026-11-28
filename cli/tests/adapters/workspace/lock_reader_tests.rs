// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::lock_reader::LockReader;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::lock_edge::LockEdge;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;

const REGISTRY: &str = "registry+https://github.com/rust-lang/crates.io-index";

fn edge(dependent: &str, dependency: &str) -> LockEdge {
    let split = |text: &str| {
        let (name, version) = text.split_once(' ').expect("a crate and its version");
        (
            String::from(name),
            Version::parse(version).expect("a version"),
        )
    };
    let ((dependent, dependent_version), (dependency, dependency_version)) =
        (split(dependent), split(dependency));
    LockEdge {
        dependent,
        dependent_version,
        dependency,
        dependency_version,
    }
}

fn entry(name: &str, version: &str) -> LockEntry {
    LockEntry {
        name: String::from(name),
        version: Version::parse(version).expect("a version"),
    }
}

#[test]
fn edges_of_a_lock_are_every_dependency_it_writes_with_a_version_on_a_crates_io_package() {
    // Arrange
    let lock = format!(
        "version = 4\n\n\
        [[package]]\nname = \"app\"\nversion = \"0.1.0\"\ndependencies = [\n \"errno\",\n \"windows-sys 0.52.0\",\n]\n\n\
        [[package]]\nname = \"errno\"\nversion = \"0.3.14\"\nsource = \"{REGISTRY}\"\ndependencies = [\n \"libc\",\n \"windows-sys 0.61.2 ({REGISTRY})\",\n]\n\n\
        [[package]]\nname = \"gitcore\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/example/upstream?branch=main#9625c3d283b3313139f8aad4ebbb0dfd3bdb09c7\"\ndependencies = [\n \"windows-sys 0.52.0 ({REGISTRY})\",\n \"windows-sys 0.61.2 (git+https://github.com/example/windows-sys?branch=main#0abc1234)\",\n]\n\n\
        [[package]]\nname = \"libc\"\nversion = \"0.2.177\"\nsource = \"{REGISTRY}\"\n\n\
        [[package]]\nname = \"windows-sys\"\nversion = \"0.52.0\"\nsource = \"{REGISTRY}\"\n\n\
        [[package]]\nname = \"windows-sys\"\nversion = \"0.61.2\"\nsource = \"{REGISTRY}\"\n\n\
        [[package]]\nname = \"windows-sys\"\nversion = \"0.61.2\"\nsource = \"git+https://github.com/example/windows-sys?branch=main#0abc1234\"\n"
    );

    // Act
    let edges = LockReader.edges(&lock);

    // Assert
    assert_eq!(
        edges,
        Ok(vec![
            edge("app 0.1.0", "windows-sys 0.52.0"),
            edge("errno 0.3.14", "windows-sys 0.61.2"),
            edge("gitcore 0.1.0", "windows-sys 0.52.0"),
        ])
    );
}

#[test]
fn edges_of_a_package_whose_version_is_no_version_are_unreadable() {
    // Arrange & Act
    let edges = LockReader.edges(&format!(
        "version = 4\n\n[[package]]\nname = \"app\"\nversion = \"four\"\ndependencies = [\n \"itoa 1.0.10\",\n]\n\n[[package]]\nname = \"itoa\"\nversion = \"1.0.10\"\nsource = \"{REGISTRY}\"\n",
    ));

    // Assert
    assert_eq!(
        edges,
        Err(PortFailure::WorkspaceUnreadable {
            file: String::from("Cargo.lock"),
            detail: String::from("the package `app` is at `four`, which is not a version"),
        })
    );
}

#[test]
fn edges_of_the_recorded_lock_are_none_as_it_holds_every_crate_once() {
    // Arrange & Act
    let edges = LockReader.edges(include_str!(
        "../../fixtures/workspace/files/Cargo.lock.txt"
    ));

    // Assert
    assert_eq!(edges, Ok(Vec::new()));
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

#[test]
fn git_of_a_lock_is_every_git_package_at_its_source_s_short_rev() {
    // Arrange & Act
    let git = LockReader.git(
        "version = 4\n\n[[package]]\nname = \"gitmacros\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/example/upstream?branch=main#9625c3d283b3313139f8aad4ebbb0dfd3bdb09c7\"\n\n[[package]]\nname = \"itoa\"\nversion = \"1.0.10\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n\n[[package]]\nname = \"gitcore\"\nversion = \"0.1.0\"\nsource = \"git+https://github.com/example/upstream?branch=main#9625c3d283b3313139f8aad4ebbb0dfd3bdb09c7\"\n",
    );

    // Assert
    assert_eq!(
        git,
        Ok(["gitcore", "gitmacros"]
            .map(|name| GitEntry {
                name: String::from(name),
                source: String::from("https://github.com/example/upstream?branch=main"),
                rev: String::from("9625c3d2"),
            })
            .to_vec())
    );
}

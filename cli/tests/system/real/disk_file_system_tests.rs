// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::model::file_failure::FileFailure;
use cargo_ripe4rust::system::real::disk_file_system::DiskFileSystem;
use cargo_ripe4rust::system::traits::file_system::FileSystem;
use tempfile::tempdir;

#[test]
fn entries_of_a_directory_are_its_names_in_order() {
    // Arrange
    let directory = tempdir().expect("a directory");
    for name in ["2026-11-26", "2026-11-24", "2026-11-25"] {
        DiskFileSystem
            .write(&directory.path().join(name).join("index"), "")
            .expect("written");
    }

    // Act
    let entries = DiskFileSystem.entries(directory.path());

    // Assert
    assert_eq!(
        entries,
        Ok(vec![
            String::from("2026-11-24"),
            String::from("2026-11-25"),
            String::from("2026-11-26"),
        ])
    );
}

#[test]
fn entries_of_a_missing_directory_are_missing() {
    // Arrange
    let directory = tempdir().expect("a directory");

    // Act
    let entries = DiskFileSystem.entries(&directory.path().join("absent"));

    // Assert
    assert_eq!(entries, Err(FileFailure::Missing));
}

#[test]
fn read_of_a_missing_file_is_missing() {
    // Arrange
    let directory = tempdir().expect("a directory");

    // Act
    let read = DiskFileSystem.read(&directory.path().join("rust-toolchain.toml"));

    // Assert
    assert_eq!(read, Err(FileFailure::Missing));
}

#[test]
fn read_of_a_written_file_is_its_text() {
    // Arrange
    let directory = tempdir().expect("a directory");
    let path = directory.path().join("ripe4rust/2026-11-26/index/smallvec");
    DiskFileSystem
        .write(&path, "{\"name\":\"smallvec\"}\n")
        .expect("written");

    // Act
    let read = DiskFileSystem.read(&path);

    // Assert
    assert_eq!(read, Ok(String::from("{\"name\":\"smallvec\"}\n")));
}

#[test]
fn write_leaves_no_partial_file_beside_it() {
    // Arrange
    let directory = tempdir().expect("a directory");

    // Act
    DiskFileSystem
        .write(&directory.path().join("either.json"), "{}")
        .expect("written");

    // Assert
    assert_eq!(
        DiskFileSystem.entries(directory.path()),
        Ok(vec![String::from("either.json")])
    );
}

#[test]
fn write_over_a_directory_fails_with_what_the_system_said() {
    // Arrange
    let directory = tempdir().expect("a directory");
    DiskFileSystem
        .write(&directory.path().join("taken/inner"), "")
        .expect("written");

    // Act
    let written = DiskFileSystem.write(&directory.path().join("taken"), "{}");

    // Assert
    assert!(
        matches!(written, Err(FileFailure::Failed { .. })),
        "{written:?}"
    );
}

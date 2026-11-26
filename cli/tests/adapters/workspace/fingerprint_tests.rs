// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::fingerprint::Fingerprint;

fn files(lock: &str) -> Vec<(String, Option<String>)> {
    vec![
        (String::from("Cargo.lock"), Some(String::from(lock))),
        (
            String::from("Cargo.toml"),
            Some(String::from("[workspace]\n")),
        ),
        (String::from("rust-toolchain.toml"), None),
    ]
}

#[test]
fn of_a_changed_file_changes() {
    // Arrange & Act
    let (before, after) = (
        Fingerprint.of(&files("version = 4\n")),
        Fingerprint.of(&files("version = 3\n")),
    );

    // Assert
    assert_ne!(before, after);
}

#[test]
fn of_contents_swapped_between_names_changes() {
    // Arrange
    let swapped = vec![
        (
            String::from("Cargo.lock"),
            Some(String::from("[workspace]\n")),
        ),
        (
            String::from("Cargo.toml"),
            Some(String::from("version = 4\n")),
        ),
        (String::from("rust-toolchain.toml"), None),
    ];

    // Act
    let fingerprint = Fingerprint.of(&swapped);

    // Assert
    assert_ne!(fingerprint, Fingerprint.of(&files("version = 4\n")));
}

#[test]
fn of_files_is_sixteen_hex_digits_the_same_on_every_run() {
    // Arrange & Act
    let fingerprint = Fingerprint.of(&files("version = 4\n"));

    // Assert
    assert_eq!(fingerprint.len(), 16);
    assert!(fingerprint.chars().all(|digit| digit.is_ascii_hexdigit()));
    assert_eq!(fingerprint, Fingerprint.of(&files("version = 4\n")));
}

#[test]
fn of_no_bytes_is_the_fnv_offset_basis() {
    // Arrange & Act
    let fingerprint = Fingerprint.of(&[]);

    // Assert
    assert_eq!(fingerprint, "cbf29ce484222325");
}

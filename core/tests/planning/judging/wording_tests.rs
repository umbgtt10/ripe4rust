// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::judging::wording::Wording;

#[test]
fn count_of_one_is_singular() {
    // Arrange & Act
    let counted = Wording.count(1, "release");

    // Assert
    assert_eq!(counted, "1 release");
}

#[test]
fn count_of_several_is_plural() {
    // Arrange & Act
    let counted = Wording.count(42, "line");

    // Assert
    assert_eq!(counted, "42 lines");
}

#[test]
fn list_joins_with_commas() {
    // Arrange
    let items = [
        String::from("RUSTSEC-2026-0190"),
        String::from("RUSTSEC-2026-0191"),
    ];

    // Act
    let listed = Wording.list(&items);

    // Assert
    assert_eq!(listed, "RUSTSEC-2026-0190, RUSTSEC-2026-0191");
}

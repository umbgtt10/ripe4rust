// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::edge::Edge;
use ripe4rust_core::values::version::Version;

#[test]
fn flipped_of_an_exclusive_edge_is_inclusive() {
    // Arrange & Act
    let flipped = Edge::Exclusive(Version::new(1, 3, 0)).flipped();

    // Assert
    assert_eq!(flipped, Edge::Inclusive(Version::new(1, 3, 0)));
}

#[test]
fn flipped_of_an_inclusive_edge_is_exclusive() {
    // Arrange & Act
    let flipped = Edge::Inclusive(Version::new(1, 2, 3)).flipped();

    // Assert
    assert_eq!(flipped, Edge::Exclusive(Version::new(1, 2, 3)));
}

#[test]
fn flipped_of_an_open_edge_stays_open() {
    // Arrange & Act
    let flipped = Edge::Open.flipped();

    // Assert
    assert_eq!(flipped, Edge::Open);
}

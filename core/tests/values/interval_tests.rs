// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::edge::Edge;
use ripe4rust_core::values::interval::Interval;
use ripe4rust_core::values::version::Version;

#[test]
fn admits_of_an_exclusive_lower_edge_refuses_the_edge() {
    // Arrange
    let interval = Interval::new(Edge::Exclusive(Version::new(1, 0, 0)), Edge::Open);

    // Act & Assert
    assert!(!interval.admits(&Version::new(1, 0, 0)));
    assert!(interval.admits(&Version::new(1, 0, 1)));
}

#[test]
fn admits_of_an_inclusive_upper_edge_admits_the_edge() {
    // Arrange
    let interval = Interval::new(Edge::Open, Edge::Inclusive(Version::new(1, 0, 0)));

    // Act & Assert
    assert!(interval.admits(&Version::new(1, 0, 0)));
    assert!(!interval.admits(&Version::new(1, 0, 1)));
}

#[test]
fn admits_of_two_open_edges_admits_everything() {
    // Arrange
    let interval = Interval::new(Edge::Open, Edge::Open);

    // Act & Assert
    assert!(interval.admits(&Version::new(0, 0, 0)));
}

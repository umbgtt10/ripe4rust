// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::at;
use crate::support::surveyed::surveyed;
use ripe4rust_core::planning::judging::facts::footprints::Footprints;
use ripe4rust_core::planning::judging::footprint::Footprint;

#[test]
fn footprint_counts_lines_and_files() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let footprint = Footprints::new(surveyed.scan.as_ref()).footprint(&[String::from("widget")]);

    // Assert
    assert_eq!(footprint, Some(Footprint { lines: 3, files: 2 }));
}

#[test]
fn footprint_of_sources_not_yet_scanned_is_unknown() {
    // Arrange & Act
    let footprint = Footprints::new(None).footprint(&[String::from("widget")]);

    // Assert
    assert_eq!(footprint, None);
}

#[test]
fn locations_of_a_group_are_its_members_together_in_order() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let locations = Footprints::new(surveyed.scan.as_ref())
        .locations(&[String::from("widget"), String::from("gadget")]);

    // Assert
    assert_eq!(
        locations,
        vec![
            at("src/lib.rs", 3),
            at("src/lib.rs", 9),
            at("src/main.rs", 2),
            at("src/net.rs", 4)
        ]
    );
}

#[test]
fn locations_of_sources_not_yet_scanned_are_none() {
    // Arrange & Act
    let locations = Footprints::new(None).locations(&[String::from("widget")]);

    // Assert
    assert_eq!(locations, Vec::new());
}

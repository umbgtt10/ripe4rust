// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::added;
use crate::support::units::moved;
use crate::support::units::removed;
use ripe4rust_core::planning::units::crate_change::CrateChange;
use ripe4rust_core::values::version::Version;

fn change(left: &[&str], reached: &[&str]) -> CrateChange {
    CrateChange::new(versions(left), versions(reached))
}

fn versions(texts: &[&str]) -> Vec<Version> {
    texts.iter().map(|text| version(text)).collect()
}

#[test]
fn is_empty_of_a_crate_no_move_names_is_true() {
    // Arrange
    let moves = [moved("memchr", "2.7.5", "2.7.6")];

    // Act
    let empty = CrateChange::of("rand", &moves).is_empty();

    // Assert
    assert!(empty);
}

#[test]
fn new_of_versions_in_any_order_holds_each_once_in_order() {
    // Arrange
    let (left, reached) = (
        versions(&["0.9.4", "0.8.6", "0.9.4"]),
        versions(&["0.10.4", "0.8.8"]),
    );

    // Act
    let made = CrateChange::new(left, reached);

    // Assert
    assert_eq!(made, change(&["0.8.6", "0.9.4"], &["0.8.8", "0.10.4"]));
}

#[test]
fn of_a_crate_gathers_the_versions_its_moves_leave_and_reach_in_order() {
    // Arrange
    let moves = [
        removed("rand", "0.9.4"),
        moved("rand", "0.8.6", "0.8.8"),
        added("rand", "0.10.4"),
        moved("memchr", "2.7.5", "2.7.6"),
    ];

    // Act
    let gathered = CrateChange::of("rand", &moves);

    // Assert
    assert_eq!(gathered, change(&["0.8.6", "0.9.4"], &["0.8.8", "0.10.4"]));
}

#[test]
fn overlaps_of_two_changes_leaving_one_version_is_true() {
    // Arrange
    let (first, second) = (change(&["2.7.5"], &["2.7.6"]), change(&["2.7.5"], &[]));

    // Act
    let overlapping = first.overlaps(&second);

    // Assert
    assert!(overlapping);
}

#[test]
fn overlaps_of_two_changes_leaving_versions_apart_is_false() {
    // Arrange
    let (first, second) = (
        change(&["0.9.4"], &["0.10.4"]),
        change(&["0.8.6"], &["0.8.8"]),
    );

    // Act
    let overlapping = first.overlaps(&second);

    // Assert
    assert!(!overlapping);
}

#[test]
fn union_of_changes_leaves_and_reaches_what_each_does() {
    // Arrange
    let changes = [
        change(&["0.9.4"], &["0.10.4"]),
        change(&["0.8.6"], &["0.8.8"]),
    ];

    // Act
    let together = CrateChange::union(&changes);

    // Assert
    assert_eq!(together, change(&["0.8.6", "0.9.4"], &["0.8.8", "0.10.4"]));
}

#[test]
fn within_of_a_change_leaving_a_version_the_other_keeps_is_false() {
    // Arrange
    let (unit, whole) = (change(&["6.0.0"], &[]), change(&["5.3.0"], &[]));

    // Act
    let inside = unit.within(&whole);

    // Assert
    assert!(!inside);
}

#[test]
fn within_of_a_change_reaching_a_version_the_other_does_not_is_false() {
    // Arrange
    let (unit, whole) = (change(&["1.0.3"], &["1.0.4"]), change(&["1.0.3"], &[]));

    // Act
    let inside = unit.within(&whole);

    // Assert
    assert!(!inside);
}

#[test]
fn within_of_a_change_the_other_holds_whole_is_true() {
    // Arrange
    let (unit, whole) = (
        change(&["0.9.4"], &["0.10.4"]),
        change(&["0.8.6", "0.9.4"], &["0.8.8", "0.10.4"]),
    );

    // Act
    let inside = unit.within(&whole);

    // Assert
    assert!(inside);
}

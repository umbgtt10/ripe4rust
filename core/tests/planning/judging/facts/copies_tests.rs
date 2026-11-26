// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::surveyed::surveyed;
use crate::support::units::breaking;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::planning::judging::facts::copies::Copies;
use ripe4rust_core::values::requirement::Requirement;

fn second_copy() -> LockedPackage {
    LockedPackage {
        name: String::from("widget"),
        version: version("2.0.1"),
        dependencies: Vec::new(),
    }
}

#[test]
fn removed_of_a_copy_another_crate_still_uses_is_false() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.workspace.lock.push(second_copy());
    surveyed.workspace.lock.push(LockedPackage {
        name: String::from("frame"),
        version: version("0.4.0"),
        dependencies: vec![ResolvedDependency {
            name: String::from("widget"),
            version: version("1.4.0"),
            requirement: Requirement::parse("1.4").expect("a requirement"),
        }],
    });

    // Act
    let removed = Copies::new(&surveyed.workspace).removed(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert!(!removed);
}

#[test]
fn removed_of_a_migration_onto_a_line_already_locked_is_true() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.workspace.lock.push(second_copy());

    // Act
    let removed = Copies::new(&surveyed.workspace).removed(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert!(removed);
}

#[test]
fn removed_with_no_second_copy_is_false() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let removed = Copies::new(&surveyed.workspace).removed(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert!(!removed);
}

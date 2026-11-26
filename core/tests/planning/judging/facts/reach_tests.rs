// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::surveyed;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::facts::reach::Reach;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

const TARGET: &str = "thumbv7em-none-eabihf";

#[test]
fn targets_of_a_unit_moving_a_crate_the_target_reaches_name_it() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.reached = vec![(String::from(TARGET), vec![String::from("gizmo")])];
    let unit = compatible(
        "widget",
        vec![
            moved("widget", "1.4.0", "1.4.1"),
            moved("gizmo", "2.7.5", "2.7.6"),
        ],
    );

    // Act
    let targets = Reach::new(&surveyed).targets(&unit);

    // Assert
    assert_eq!(targets, vec![String::from(TARGET)]);
}

#[test]
fn targets_of_a_unit_the_target_does_not_reach_are_none() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.reached = vec![(String::from(TARGET), vec![String::from("libc")])];
    let unit = compatible("widget", vec![moved("widget", "1.4.0", "1.4.1")]);

    // Act
    let targets = Reach::new(&surveyed).targets(&unit);

    // Assert
    assert_eq!(targets, Vec::<String>::new());
}

#[test]
fn targets_of_the_toolchain_are_every_target() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.policy.validate_beyond_gates = vec![String::from(TARGET)];
    let mut unit = compatible("rust", Vec::new());
    unit.kind = UnitKind::Toolchain;
    unit.id = UnitId::of(UnitKind::Toolchain, "rust");

    // Act
    let targets = Reach::new(&surveyed).targets(&unit);

    // Assert
    assert_eq!(targets, vec![String::from(TARGET)]);
}

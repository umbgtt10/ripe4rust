// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::at;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::writing::edit_finder::EditFinder;
use ripe4rust_core::planning::writing::edits::Edits;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn edits_of_a_breaking_unit_write_its_requirement_and_the_rust_version_once() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let edits = EditFinder::new(&context).edits(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(
        edits,
        vec![
            Edits.set(at("Cargo.toml", 21), "widget", String::from("2.1.0")),
            Edits.set(at("Cargo.toml", 5), "rust-version", String::from("1.88")),
        ]
    );
}

#[test]
fn edits_of_a_compatible_unit_are_none() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let edits = EditFinder::new(&context).edits(&compatible(
        "widget",
        vec![moved("widget", "1.4.0", "1.4.1")],
    ));

    // Assert
    assert_eq!(edits, Vec::new());
}

#[test]
fn edits_of_the_toolchain_write_its_channel() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = breaking("rust", "1.99.1", "1.100.0");
    unit.kind = UnitKind::Toolchain;

    // Act
    let edits = EditFinder::new(&context).edits(&unit);

    // Assert
    assert_eq!(
        edits,
        vec![Edits.set(
            at("rust-toolchain.toml", 9),
            "channel",
            String::from("1.100.0")
        )]
    );
}

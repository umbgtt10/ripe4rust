// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::needing;
use crate::support::releases::version;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use ripe4rust_core::planning::judging::facts::held_back::HeldBack;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn versions_of_another_unit_are_none() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let versions = HeldBack::new(&context).versions(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(versions, Vec::<String>::new());
}

#[test]
fn versions_of_the_toolchain_are_what_it_would_release() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.releases[3]
        .1
        .push(needing("2.2.0", "2026-10-20", "1.100"));
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let unit = Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: Some(version("1.99.1")),
        to: Some(version("1.100.0")),
        moves: Vec::new(),
        held: Vec::new(),
    };

    // Act
    let versions = HeldBack::new(&context).versions(&unit);

    // Assert
    assert_eq!(versions, vec![String::from("widget 2.2.0")]);
}

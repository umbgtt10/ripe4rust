// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::release;
use crate::support::releases::version;
use ripe4rust_core::planning::units::choice::release_history::ReleaseHistory;
use ripe4rust_core::planning::units::toolchain_unit::ToolchainUnit;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::version::Version;

fn stable() -> ReleaseHistory {
    ReleaseHistory::new(
        vec![
            release("1.99.0", "2026-08-27"),
            release("1.99.1", "2026-09-10"),
            release("1.100.0", "2026-10-08"),
        ],
        Version::toolchain_line,
    )
}

#[test]
fn build_of_an_older_pinned_release_moves_the_channel_to_the_newest() {
    // Arrange
    let history = stable();

    // Act
    let unit = ToolchainUnit::new(&history).build(Some(&version("1.99.1")));

    // Assert
    assert_eq!(
        unit,
        Some(Unit {
            id: UnitId::of(UnitKind::Toolchain, "rust"),
            kind: UnitKind::Toolchain,
            name: Some(String::from("rust")),
            members: vec![String::from("rust")],
            from: Some(version("1.99.1")),
            to: Some(version("1.100.0")),
            moves: Vec::new(),
            held: Vec::new(),
        })
    );
}

#[test]
fn build_of_no_pinned_release_is_none() {
    // Arrange
    let history = stable();

    // Act
    let unit = ToolchainUnit::new(&history).build(None);

    // Assert
    assert_eq!(unit, None);
}

#[test]
fn build_of_the_newest_release_is_none() {
    // Arrange
    let history = stable();

    // Act
    let unit = ToolchainUnit::new(&history).build(Some(&version("1.100.0")));

    // Assert
    assert_eq!(unit, None);
}

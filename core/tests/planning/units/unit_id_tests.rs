// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn fmt_of_an_advisory_row_names_the_advisory() {
    // Arrange & Act
    let id = UnitId::of(UnitKind::Advisory, "RUSTSEC-2026-0190").to_string();

    // Assert
    assert_eq!(id, "advisory:RUSTSEC-2026-0190");
}

#[test]
fn fmt_of_the_toolchain_is_toolchain_rust() {
    // Arrange & Act
    let id = UnitId::of(UnitKind::Toolchain, "rust").to_string();

    // Assert
    assert_eq!(id, "toolchain:rust");
}

#[test]
fn of_writes_kind_colon_crate() {
    // Arrange & Act
    let id = UnitId::of(UnitKind::Compatible, "clap");

    // Assert
    assert_eq!(id.to_string(), "compatible:clap");
}

#[test]
fn rest_is_rest() {
    // Arrange & Act
    let id = UnitId::rest();

    // Assert
    assert_eq!(id.to_string(), "rest");
}

#[test]
fn split_names_one_move_of_the_unit() {
    // Arrange
    let id = UnitId::of(UnitKind::Compatible, "clap");

    // Act
    let split = id.split("clap_builder");

    // Assert
    assert_eq!(split, "compatible:clap/clap_builder");
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::units::own_move::OwnMove;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;

fn held() -> Unit {
    Unit {
        to: None,
        moves: vec![moved("libc", "0.2.177", "0.2.178")],
        held: vec![moved("memchr", "2.5.0", "2.7.6")],
        ..compatible("memchr", vec![moved("memchr", "2.5.0", "2.7.6")])
    }
}

fn rest() -> Unit {
    Unit {
        id: UnitId::rest(),
        name: None,
        members: Vec::new(),
        ..compatible("rest", vec![moved("mio", "1.0.4", "1.0.5")])
    }
}

fn staying() -> Unit {
    Unit {
        from: Some(version("1.48.0")),
        ..compatible("tokio", vec![moved("mio", "1.0.4", "1.0.5")])
    }
}

#[test]
fn held_of_a_unit_whose_own_move_a_pin_holds_is_true() {
    // Arrange & Act
    let held = [OwnMove.held(&held()), OwnMove.held(&staying())];

    // Assert
    assert_eq!(held, [true, false]);
}

#[test]
fn lands_on_of_a_unit_moving_its_crate_is_the_version_it_takes() {
    // Arrange
    let unit = breaking("widget", "1.4.0", "2.0.1");

    // Act
    let landed = OwnMove.lands_on(&unit);

    // Assert
    assert_eq!(landed, Some(&version("2.0.1")));
}

#[test]
fn lands_on_of_a_unit_whose_crate_stays_is_the_version_in_use() {
    // Arrange
    let (staying, held, rest) = (staying(), held(), rest());

    // Act
    let landed = [
        OwnMove.lands_on(&staying),
        OwnMove.lands_on(&held),
        OwnMove.lands_on(&rest),
    ];

    // Assert
    assert_eq!(landed, [Some(&version("1.48.0")), None, None]);
}

#[test]
fn stays_of_a_compatible_unit_that_moves_only_what_its_crate_pulls_in_is_true() {
    // Arrange & Act
    let stays = OwnMove.stays(&staying());

    // Assert
    assert!(stays);
}

#[test]
fn stays_of_a_unit_moving_its_crate_held_or_the_rest_is_false() {
    // Arrange
    let moving = compatible("widget", vec![moved("widget", "1.0.0", "1.0.1")]);

    // Act
    let stays = [
        OwnMove.stays(&moving),
        OwnMove.stays(&held()),
        OwnMove.stays(&rest()),
        OwnMove.stays(&breaking("widget", "1.4.0", "2.0.1")),
    ];

    // Assert
    assert_eq!(stays, [false, false, false, false]);
}

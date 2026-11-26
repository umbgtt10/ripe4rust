// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::compatible;
use crate::support::plans::moved;
use crate::support::plans::rest;
use cargo_ripe4rust::render::table::target_cell::TargetCell;

#[test]
fn text_of_a_unit_moving_its_crate_is_the_version_it_takes() {
    // Arrange & Act
    let text = TargetCell.text(&compatible().unit);

    // Assert
    assert_eq!(text, "4.6.7");
}

#[test]
fn text_of_a_unit_whose_crate_stays_says_so() {
    // Arrange
    let mut unit = compatible().unit;
    unit.to = None;
    unit.moves = vec![moved("widget_builder", Some("4.6.6"), Some("4.6.7"))];

    // Act
    let text = TargetCell.text(&unit);

    // Assert
    assert_eq!(text, "stays");
}

#[test]
fn text_of_a_unit_whose_own_move_is_held_says_so() {
    // Arrange
    let mut unit = compatible().unit;
    unit.to = None;
    unit.moves = vec![moved("widget_builder", Some("4.6.6"), Some("4.6.7"))];
    unit.held = vec![moved("widget", Some("4.6.6"), Some("4.6.7"))];

    // Act
    let text = TargetCell.text(&unit);

    // Assert
    assert_eq!(text, "held");
}

#[test]
fn text_of_the_rest_is_none() {
    // Arrange & Act
    let text = TargetCell.text(&rest().unit);

    // Assert
    assert_eq!(text, "--");
}

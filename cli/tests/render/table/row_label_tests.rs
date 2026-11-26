// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::breaking;
use crate::support::plans::compatible;
use crate::support::plans::rest;
use cargo_ripe4rust::render::table::row_label::RowLabel;

#[test]
fn unit_of_a_breaking_unit_is_its_crate() {
    // Arrange & Act
    let label = RowLabel.unit(&breaking());

    // Assert
    assert_eq!(label, "gadget");
}

#[test]
fn unit_of_a_compatible_unit_counts_what_it_pulls_in() {
    // Arrange & Act
    let label = RowLabel.unit(&compatible());

    // Assert
    assert_eq!(label, "widget (+1 transitive)");
}

#[test]
fn unit_of_a_group_counts_its_other_members() {
    // Arrange
    let mut group = compatible();
    group.unit.members.push(String::from("widget_builder"));

    // Act
    let label = RowLabel.unit(&group);

    // Assert
    assert_eq!(label, "widget (+1)");
}

#[test]
fn unit_of_the_rest_is_the_rest() {
    // Arrange & Act
    let label = RowLabel.unit(&rest());

    // Assert
    assert_eq!(label, "the rest");
}

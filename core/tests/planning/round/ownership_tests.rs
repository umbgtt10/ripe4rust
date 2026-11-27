// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::plan_with_shared;
use crate::support::units::moved;
use crate::support::units::removed;
use ripe4rust_core::inputs::recorded_unit::RecordedUnit;
use ripe4rust_core::planning::round::ownership::Ownership;

fn unit(id: &str) -> RecordedUnit {
    plan_with_shared()
        .units
        .into_iter()
        .find(|unit| unit.id == id)
        .expect("the unit")
}

#[test]
fn is_shared_of_a_crate_the_plan_names_is_true() {
    // Arrange
    let plan = plan_with_shared();

    // Act
    let shared = ["wasip2", "ahash"].map(|name| Ownership::new(&plan.shared).is_shared(name));

    // Assert
    assert_eq!(shared, [true, false]);
}

#[test]
fn owned_of_a_unit_is_the_moves_it_owns() {
    // Arrange
    let plan = plan_with_shared();

    // Act
    let owned = Ownership::new(&plan.shared).owned(&unit("compatible:tonic"));

    // Assert
    assert_eq!(owned, vec![moved("h2", "0.4.20", "0.4.21")]);
}

#[test]
fn owned_of_a_unit_owning_none_of_its_moves_is_them_all() {
    // Arrange
    let mut plan = plan_with_shared();
    plan.shared.push(String::from("h2"));

    // Act
    let owned = Ownership::new(&plan.shared).owned(&unit("compatible:tonic"));

    // Assert
    assert_eq!(
        owned,
        vec![moved("h2", "0.4.20", "0.4.21"), removed("r-efi", "6.0.0")]
    );
}

#[test]
fn owns_a_move_of_a_crate_the_units_do_not_share() {
    // Arrange
    let plan = plan_with_shared();

    // Act
    let owned = Ownership::new(&plan.shared).owns(
        &unit("compatible:boon"),
        &moved("ahash", "0.8.12", "0.8.14"),
    );

    // Assert
    assert!(owned);
}

#[test]
fn owns_no_move_of_a_crate_the_units_share_another_unit_s_moves_can_take_on() {
    // Arrange
    let plan = plan_with_shared();

    // Act
    let owned = Ownership::new(&plan.shared)
        .owns(&unit("compatible:boon"), &moved("wasip2", "1.0.3", "1.0.4"));

    // Assert
    assert!(!owned);
}

#[test]
fn owns_the_move_of_its_own_crate_though_the_units_share_it() {
    // Arrange
    let mut plan = plan_with_shared();
    plan.shared.push(String::from("proptest"));

    // Act
    let owned = Ownership::new(&plan.shared).owns(
        &unit("compatible:proptest"),
        &moved("proptest", "1.11.0", "1.12.0"),
    );

    // Assert
    assert!(owned);
}

#[test]
fn owns_the_rest_s_move_its_step_takes_alone_but_not_a_crate_only_removed() {
    // Arrange
    let plan = plan_with_shared();
    let ownership = Ownership::new(&plan.shared);

    // Act
    let owned = [
        moved("libc", "0.2.177", "0.2.178"),
        removed("getrandom", "0.3.4"),
    ]
    .map(|lock_move| ownership.owns(&unit("rest"), &lock_move));

    // Assert
    assert_eq!(owned, [true, false]);
}

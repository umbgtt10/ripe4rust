// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::lock;
use crate::support::recorded::plan_with_shared;
use crate::support::recorded::shared_base;
use crate::support::units::moved;
use crate::support::units::removed;
use ripe4rust_core::planning::round::checking::accountable::Accountable;
use ripe4rust_core::planning::round::lock_changes::LockChanges;
use ripe4rust_core::planning::round::ownership::Ownership;
use ripe4rust_core::planning::round::target::Target;
use ripe4rust_core::planning::round::target_finder::TargetFinder;

fn target(id: &str) -> Target {
    TargetFinder::new(&plan_with_shared())
        .find(id)
        .expect("a unit")
}

#[test]
fn answers_for_a_crate_the_units_do_not_share_or_its_own() {
    // Arrange
    let plan = plan_with_shared();
    let target = target("compatible:boon");
    let lock = LockChanges::new(&plan.lock, &plan.lock);
    let accountable = Accountable::new(&target, Ownership::new(&plan.shared), &lock);

    // Act
    let answered = ["ahash", "wasip2", "boon"].map(|name| accountable.answers_for(name));

    // Assert
    assert_eq!(answered, [true, false, true]);
}

#[test]
fn answers_for_its_split_s_crate_though_the_units_share_it() {
    // Arrange
    let plan = plan_with_shared();
    let target = target("compatible:boon/wasip2");
    let lock = LockChanges::new(&plan.lock, &plan.lock);
    let accountable = Accountable::new(&target, Ownership::new(&plan.shared), &lock);

    // Act
    let answered = accountable.answers_for("wasip2");

    // Assert
    assert!(answered);
}

#[test]
fn moves_a_crate_its_moves_name_and_no_other() {
    // Arrange
    let plan = plan_with_shared();
    let target = target("compatible:tonic");
    let lock = LockChanges::new(&plan.lock, &plan.lock);
    let accountable = Accountable::new(&target, Ownership::new(&plan.shared), &lock);

    // Act
    let moved = ["r-efi", "wasip2"].map(|name| accountable.moves(name));

    // Assert
    assert_eq!(moved, [true, false]);
}

#[test]
fn owns_a_split_s_move_of_a_shared_crate_and_not_the_unit_s() {
    // Arrange
    let plan = plan_with_shared();
    let (split, whole) = (target("compatible:boon/wasip2"), target("compatible:boon"));
    let lock = LockChanges::new(&plan.lock, &plan.lock);
    let wasip2 = moved("wasip2", "1.0.3", "1.0.4");

    // Act
    let owned = [&split, &whole]
        .map(|target| Accountable::new(target, Ownership::new(&plan.shared), &lock).owns(&wasip2));

    // Assert
    assert_eq!(owned, [true, false]);
}

#[test]
fn unmade_of_a_unit_is_its_moves_the_lock_does_not_show() {
    // Arrange
    let plan = plan_with_shared();
    let target = target("compatible:tonic");
    let mut entries = shared_base();
    entries[2] = lock(&[("h2", "0.4.21")])[0].clone();
    let lock = LockChanges::new(&plan.lock, &entries);
    let accountable = Accountable::new(&target, Ownership::new(&plan.shared), &lock);

    // Act
    let unmade = accountable.unmade();

    // Assert
    assert_eq!(unmade, vec![&removed("r-efi", "6.0.0")]);
}

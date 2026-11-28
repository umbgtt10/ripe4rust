// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::lock;
use crate::support::releases::version;
use crate::support::units::added;
use crate::support::units::moved;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::round::lock_changes::LockChanges;

fn removed(name: &str, from: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: Some(version(from)),
        to: None,
    }
}

#[test]
fn applied_of_a_move_the_lock_does_not_show_is_false() {
    // Arrange
    let base = lock(&[("widget", "1.4.0")]);
    let unmoved = LockChanges::new(&base, &base);
    let elsewhere = LockChanges::new(&base, &lock(&[("widget", "1.4.2")]));

    // Act
    let applied =
        [&unmoved, &elsewhere].map(|changes| changes.applied(&moved("widget", "1.4.0", "1.4.1")));

    // Assert
    assert_eq!(applied, [false, false]);
}

#[test]
fn applied_of_a_move_the_lock_shows_is_true() {
    // Arrange
    let changes = LockChanges::new(&lock(&[("widget", "1.4.0")]), &lock(&[("widget", "1.4.1")]));

    // Act
    let applied = changes.applied(&moved("widget", "1.4.0", "1.4.1"));

    // Assert
    assert!(applied);
}

#[test]
fn applied_of_a_move_to_a_copy_the_lock_already_held_is_its_old_copy_gone() {
    // Arrange
    let changes = LockChanges::new(
        &lock(&[("syn", "1.0.109"), ("syn", "2.0.117")]),
        &lock(&[("syn", "2.0.117")]),
    );

    // Act
    let applied = changes.applied(&moved("syn", "1.0.109", "2.0.117"));

    // Assert
    assert!(applied);
}

#[test]
fn applied_of_a_removal_is_the_package_gone() {
    // Arrange
    let changes = LockChanges::new(&lock(&[("old", "1.0.0")]), &lock(&[]));

    // Act
    let applied = changes.applied(&removed("old", "1.0.0"));

    // Assert
    assert!(applied);
}

#[test]
fn applied_of_an_addition_is_a_package_the_lock_gained() {
    // Arrange
    let gained = LockChanges::new(&lock(&[]), &lock(&[("tiny", "1.0.0")]));
    let held = LockChanges::new(&lock(&[("tiny", "1.0.0")]), &lock(&[("tiny", "1.0.0")]));

    // Act
    let applied = [&gained, &held].map(|changes| changes.applied(&added("tiny", "1.0.0")));

    // Assert
    assert_eq!(applied, [true, false]);
}

#[test]
fn holds_of_a_version_is_whether_the_lock_holds_it_now() {
    // Arrange
    let changes = LockChanges::new(&lock(&[("widget", "1.4.0")]), &lock(&[("widget", "1.4.1")]));

    // Act
    let held = ["1.4.0", "1.4.1"].map(|at| changes.holds("widget", &version(at)));

    // Assert
    assert_eq!(held, [false, true]);
}

#[test]
fn kept_of_a_version_is_whether_the_plan_s_lock_and_the_lock_now_both_hold_it() {
    // Arrange
    let changes = LockChanges::new(
        &lock(&[("windows-sys", "0.52.0"), ("windows-sys", "0.61.2")]),
        &lock(&[("windows-sys", "0.52.0"), ("windows-sys", "0.60.2")]),
    );

    // Act
    let kept = ["0.52.0", "0.60.2", "0.61.2"].map(|at| changes.kept("windows-sys", &version(at)));

    // Assert
    assert_eq!(kept, [true, false, false]);
}

#[test]
fn unexplained_of_changes_no_applied_move_accounts_for_are_paired_by_crate() {
    // Arrange
    let changes = LockChanges::new(
        &lock(&[("libc", "0.2.177"), ("old", "1.0.0"), ("widget", "1.4.0")]),
        &lock(&[("libc", "0.2.180"), ("tiny", "1.0.0"), ("widget", "1.4.1")]),
    );

    // Act
    let unexplained = changes.unexplained(&[moved("widget", "1.4.0", "1.4.1")]);

    // Assert
    assert_eq!(
        unexplained,
        vec![
            moved("libc", "0.2.177", "0.2.180"),
            removed("old", "1.0.0"),
            added("tiny", "1.0.0"),
        ]
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::plan_with_shared;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::planning::round::git_changes::GitChanges;
use ripe4rust_core::planning::round::line_changes::LineChanges;
use ripe4rust_core::planning::round::lock_changes::LockChanges;
use ripe4rust_core::planning::round::ownership::Ownership;
use ripe4rust_core::planning::round::progress::Progress;

fn base_lock() -> Vec<LockEntry> {
    plan().lock
}

#[test]
fn applied_of_a_unit_partly_applied_is_the_split_ids_of_its_moves_applied() {
    // Arrange
    let lock_changes = LockChanges::new(
        &base_lock(),
        &lock(&[
            ("gadget", "0.13.1"),
            ("gizmo", "2.7.6"),
            ("libc", "0.2.177"),
            ("widget", "1.4.0"),
        ]),
    );
    let line_changes = LineChanges::new(&lines(), &lines());
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&[]);
    let progress = Progress::new(&lock_changes, &line_changes, &git_changes, &ownership);

    // Act
    let listed = (
        progress.applied(&plan().units[2]),
        progress.to_go(&plan().units[2]),
    );

    // Assert
    assert_eq!(
        listed,
        (
            vec![String::from("compatible:widget/gizmo")],
            vec![String::from("compatible:widget/widget")]
        )
    );
}

#[test]
fn applied_of_a_unit_wholly_applied_is_its_id() {
    // Arrange
    let lock_changes = LockChanges::new(
        &base_lock(),
        &lock(&[
            ("gadget", "0.14.0"),
            ("gizmo", "2.7.5"),
            ("libc", "0.2.177"),
            ("widget", "1.4.0"),
        ]),
    );
    let line_changes = LineChanges::new(&lines(), &lines_with(&[(22, "0.14.0")]));
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&[]);
    let progress = Progress::new(&lock_changes, &line_changes, &git_changes, &ownership);

    // Act
    let listed = (
        progress.applied(&plan().units[0]),
        progress.to_go(&plan().units[0]),
    );

    // Assert
    assert_eq!(listed, (vec![String::from("breaking:gadget")], Vec::new()));
}

#[test]
fn applied_of_a_unit_whose_move_of_a_shared_crate_a_later_unit_took_on_is_its_id() {
    // Arrange
    let plan = plan_with_shared();
    let lock_changes = LockChanges::new(
        &plan.lock,
        &lock(&[
            ("ahash", "0.8.14"),
            ("h2", "0.4.20"),
            ("libc", "0.2.177"),
            ("proptest", "1.12.0"),
            ("r-efi", "5.3.0"),
            ("r-efi", "6.0.0"),
            ("rand", "0.10.4"),
        ]),
    );
    let line_changes = LineChanges::new(&lines(), &lines());
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&plan.shared);
    let progress = Progress::new(&lock_changes, &line_changes, &git_changes, &ownership);

    // Act
    let listed = (
        progress.applied(&plan.units[0]),
        progress.to_go(&plan.units[0]),
    );

    // Assert
    assert_eq!(listed, (vec![String::from("compatible:boon")], Vec::new()));
}

#[test]
fn to_go_of_a_unit_partly_applied_without_a_split_is_its_id() {
    // Arrange
    let lock_changes = LockChanges::new(&base_lock(), &base_lock());
    let line_changes = LineChanges::new(&lines(), &lines_with(&[(22, "0.14.0")]));
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&[]);
    let progress = Progress::new(&lock_changes, &line_changes, &git_changes, &ownership);

    // Act
    let listed = (
        progress.applied(&plan().units[0]),
        progress.to_go(&plan().units[0]),
    );

    // Assert
    assert_eq!(listed, (Vec::new(), vec![String::from("breaking:gadget")]));
}

#[test]
fn to_go_of_a_unit_untouched_is_its_id() {
    // Arrange
    let lock_changes = LockChanges::new(&base_lock(), &base_lock());
    let line_changes = LineChanges::new(&lines(), &lines());
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&[]);
    let progress = Progress::new(&lock_changes, &line_changes, &git_changes, &ownership);

    // Act
    let listed = (
        progress.applied(&plan().units[1]),
        progress.to_go(&plan().units[1]),
    );

    // Assert
    assert_eq!(listed, (Vec::new(), vec![String::from("toolchain:rust")]));
}

#[test]
fn whole_of_a_unit_is_every_move_and_edit_of_it_applied() {
    // Arrange
    let lock_changes = LockChanges::new(
        &base_lock(),
        &lock(&[
            ("gadget", "0.14.0"),
            ("gizmo", "2.7.5"),
            ("libc", "0.2.177"),
            ("widget", "1.4.0"),
        ]),
    );
    let made = LineChanges::new(&lines(), &lines_with(&[(22, "0.14.0")]));
    let unmade = LineChanges::new(&lines(), &lines());
    let git_changes = GitChanges::new(&[], &[]);
    let ownership = Ownership::new(&[]);

    // Act
    let whole = [&made, &unmade].map(|line_changes| {
        Progress::new(&lock_changes, line_changes, &git_changes, &ownership).whole(&plan().units[0])
    });

    // Assert
    assert_eq!(whole, [true, false]);
}

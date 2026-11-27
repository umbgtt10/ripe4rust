// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::units::added;
use crate::support::units::moved;
use crate::support::units::removed;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::units::shared_crates::SharedCrates;

fn shared(held: &[&str], claimants: &[Vec<LockMove>], workspace: &[LockMove]) -> Vec<String> {
    let claimants = claimants
        .iter()
        .map(Vec::as_slice)
        .collect::<Vec<&[LockMove]>>();
    SharedCrates::new(held.iter().map(|name| String::from(*name)).collect())
        .of(&claimants, workspace)
}

#[test]
fn of_a_crate_a_unit_moves_where_the_whole_workspace_removes_it_is_shared() {
    // Arrange
    let claimants = [
        vec![
            moved("ahash", "0.8.12", "0.8.14"),
            moved("wasip2", "1.0.3", "1.0.4"),
        ],
        vec![moved("proptest", "1.11.0", "1.12.0")],
    ];
    let workspace = [
        moved("ahash", "0.8.12", "0.8.14"),
        moved("proptest", "1.11.0", "1.12.0"),
        removed("wasip2", "1.0.3"),
    ];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, vec![String::from("wasip2")]);
}

#[test]
fn of_a_crate_a_unit_removes_where_the_whole_workspace_keeps_it_is_shared() {
    // Arrange
    let claimants = [vec![
        moved("h2", "0.4.20", "0.4.21"),
        removed("r-efi", "6.0.0"),
    ]];
    let workspace = [moved("h2", "0.4.20", "0.4.21"), removed("r-efi", "5.3.0")];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, vec![String::from("r-efi")]);
}

#[test]
fn of_a_crate_in_two_versions_each_unit_moving_its_own_is_not_shared() {
    // Arrange
    let claimants = [
        vec![moved("rand", "0.9.4", "0.10.4")],
        vec![moved("rand", "0.8.6", "0.8.8")],
    ];
    let workspace = [
        removed("rand", "0.8.6"),
        removed("rand", "0.9.4"),
        added("rand", "0.8.8"),
        added("rand", "0.10.4"),
    ];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, Vec::<String>::new());
}

#[test]
fn of_a_crate_only_the_whole_workspace_moves_is_shared() {
    // Arrange
    let claimants = [vec![moved("proptest", "1.11.0", "1.12.0")]];
    let workspace = [
        removed("getrandom", "0.3.4"),
        moved("proptest", "1.11.0", "1.12.0"),
    ];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, vec![String::from("getrandom")]);
}

#[test]
fn of_a_crate_pinned_with_hold_all_is_never_shared() {
    // Arrange
    let claimants = [vec![moved("proptest", "1.11.0", "1.12.0")]];
    let workspace = [
        removed("getrandom", "0.3.4"),
        moved("proptest", "1.11.0", "1.12.0"),
    ];

    // Act
    let crates = shared(&["getrandom"], &claimants, &workspace);

    // Assert
    assert_eq!(crates, Vec::<String>::new());
}

#[test]
fn of_a_move_every_unit_and_the_whole_workspace_agree_on_is_not_shared() {
    // Arrange
    let claimants = [
        vec![moved("memchr", "2.7.5", "2.7.6")],
        vec![moved("memchr", "2.7.5", "2.7.6")],
    ];
    let workspace = [moved("memchr", "2.7.5", "2.7.6")];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, Vec::<String>::new());
}

#[test]
fn of_several_crates_names_each_once_in_order() {
    // Arrange
    let claimants = [
        vec![moved("wasip2", "1.0.3", "1.0.4")],
        vec![removed("r-efi", "6.0.0"), moved("wasip2", "1.0.3", "1.0.4")],
    ];
    let workspace = [
        removed("getrandom", "0.3.4"),
        removed("r-efi", "5.3.0"),
        removed("wasip2", "1.0.3"),
    ];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(
        crates,
        vec![
            String::from("getrandom"),
            String::from("r-efi"),
            String::from("wasip2")
        ]
    );
}

#[test]
fn of_two_units_taking_one_version_to_two_places_is_shared() {
    // Arrange
    let claimants = [
        vec![moved("memchr", "2.7.5", "2.7.6")],
        vec![removed("memchr", "2.7.5")],
    ];
    let workspace = [moved("memchr", "2.7.5", "2.7.6")];

    // Act
    let crates = shared(&[], &claimants, &workspace);

    // Assert
    assert_eq!(crates, vec![String::from("memchr")]);
}

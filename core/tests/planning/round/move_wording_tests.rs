// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::edge;
use crate::support::releases::version;
use crate::support::units::added;
use crate::support::units::moved;
use crate::support::units::source;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::round::move_wording::MoveWording;

#[test]
fn git_of_a_source_moved_says_from_which_rev_to_which_no_unit_moves_it() {
    // Arrange & Act
    let said = MoveWording.git(&source("9625c3d2", "deadbeef"));

    // Assert
    assert_eq!(
        said,
        "`https://github.com/example/upstream?branch=main` moved 9625c3d2 -> deadbeef, a move no unit of the plan makes"
    );
}

#[test]
fn repointed_of_an_edge_says_what_its_dependency_moved_between() {
    // Arrange
    let recorded = edge(("errno", "0.3.14"), ("windows-sys", "0.61.2"));

    // Act
    let said = MoveWording.repointed(&recorded, &version("0.52.0"));

    // Assert
    assert_eq!(
        said,
        "`errno` 0.3.14's `windows-sys` 0.61.2 -> 0.52.0, both already in the lock"
    );
}

#[test]
fn shared_of_each_kind_of_move_says_the_units_share_the_crate() {
    // Arrange
    let removed = LockMove {
        name: String::from("wasip2"),
        from: Some(version("1.0.3")),
        to: None,
    };

    // Act
    let said = [
        moved("memchr", "2.7.5", "2.7.7"),
        added("getrandom", "0.4.3"),
        removed,
    ]
    .map(|lock_move| MoveWording.shared(&lock_move));

    // Assert
    assert_eq!(
        said,
        [
            String::from(
                "`memchr` moved 2.7.5 -> 2.7.7: the units share the crate, and its moves depend on the order they are applied in"
            ),
            String::from(
                "`getrandom` 0.4.3 joined the lock: the units share the crate, and its moves depend on the order they are applied in"
            ),
            String::from(
                "`wasip2` 1.0.3 left the lock: the units share the crate, and its moves depend on the order they are applied in"
            ),
        ]
    );
}

#[test]
fn unexplained_of_each_kind_of_move_says_no_unit_makes_it() {
    // Arrange
    let removed = LockMove {
        name: String::from("old"),
        from: Some(version("1.0.0")),
        to: None,
    };

    // Act
    let said = [
        moved("libc", "0.2.177", "0.2.180"),
        added("tiny", "1.0.0"),
        removed,
    ]
    .map(|lock_move| MoveWording.unexplained(&lock_move));

    // Assert
    assert_eq!(
        said,
        [
            String::from("`libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes"),
            String::from("`tiny` 1.0.0 joined the lock, which no unit of the plan adds"),
            String::from("`old` 1.0.0 left the lock, which no unit of the plan removes"),
        ]
    );
}

#[test]
fn why_shared_of_a_sentence_adds_why_it_is_no_difference() {
    // Arrange & Act
    let said =
        MoveWording.why_shared("`r-efi` 6.0.0 is still in the lock, where the unit removes it");

    // Assert
    assert_eq!(
        said,
        "`r-efi` 6.0.0 is still in the lock, where the unit removes it: the units share the crate, and its moves depend on the order they are applied in"
    );
}

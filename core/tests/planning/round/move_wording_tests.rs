// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

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

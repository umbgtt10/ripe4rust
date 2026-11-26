// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::added;
use crate::support::units::moved;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::round::move_wording::MoveWording;

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

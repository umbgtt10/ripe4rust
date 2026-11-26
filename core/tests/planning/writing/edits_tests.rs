// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::writing::edits::Edits;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::values::location::Location;

#[test]
fn set_writes_the_value_for_the_key_at_the_line() {
    // Arrange
    let at = Location {
        file: String::from("rust-toolchain.toml"),
        line: 9,
    };

    // Act
    let step = Edits.set(at.clone(), "channel", String::from("1.100.0"));

    // Assert
    assert_eq!(
        step,
        Step::Edit {
            at,
            key: String::from("channel"),
            value: String::from("1.100.0"),
        }
    );
}

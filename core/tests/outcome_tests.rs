// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::outcome::Outcome;

#[test]
fn label_of_ask_is_ask() {
    // Arrange & Act
    let label = Outcome::Ask.label();

    // Assert
    assert_eq!(label, "ask");
}

#[test]
fn label_of_ripe_is_ripe() {
    // Arrange & Act
    let label = Outcome::Ripe.label();

    // Assert
    assert_eq!(label, "ripe");
}

#[test]
fn label_of_ripening_is_ripening() {
    // Arrange & Act
    let label = Outcome::Ripening.label();

    // Assert
    assert_eq!(label, "ripening");
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::unreadable::Unreadable;
use ripe4rust_core::inputs::port_failure::PortFailure;

#[test]
fn failure_is_internal_naming_what_was_read() {
    // Arrange & Act
    let failure =
        Unreadable::new("reading what `cargo update --dry-run` printed").failure("a line");

    // Assert
    assert_eq!(
        failure,
        PortFailure::Internal {
            doing: String::from("reading what `cargo update --dry-run` printed"),
            detail: String::from("a line"),
        }
    );
}

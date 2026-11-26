// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::unreadable::Unreadable;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::version::Version;

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

#[test]
fn unparsed_names_the_text_and_what_it_should_have_been() {
    // Arrange
    let failure = Version::parse("one").expect_err("not a version");

    // Act
    let unparsed = Unreadable::new("reading the index's answer for `itoa`").unparsed(&failure);

    // Assert
    assert_eq!(
        unparsed,
        PortFailure::Internal {
            doing: String::from("reading the index's answer for `itoa`"),
            detail: String::from("`one` is not a version"),
        }
    );
}

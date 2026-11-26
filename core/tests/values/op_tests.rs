// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::op::Op;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;

fn partial(text: &str) -> PartialVersion {
    PartialVersion::parse(text).expect("a partial version")
}

#[test]
fn interval_of_a_caret_stops_at_its_ceiling() {
    // Arrange
    let interval = Op::Caret.interval(&partial("1.2"));

    // Act & Assert
    assert!(interval.admits(&Version::new(1, 2, 0)));
    assert!(!interval.admits(&Version::new(2, 0, 0)));
}

#[test]
fn interval_of_a_less_stops_before_its_floor() {
    // Arrange
    let interval = Op::Less.interval(&partial("1.2.3"));

    // Act & Assert
    assert!(interval.admits(&Version::new(1, 2, 2)));
    assert!(!interval.admits(&Version::new(1, 2, 3)));
}

#[test]
fn split_of_a_bare_version_is_a_caret_on_all_of_it() {
    // Arrange & Act
    let split = Op::split("1.2.3");

    // Assert
    assert_eq!(split, (Op::Caret, "1.2.3"));
}

#[test]
fn split_of_a_lower_bound_takes_both_characters() {
    // Arrange & Act
    let split = Op::split(">=0.32");

    // Assert
    assert_eq!(split, (Op::GreaterEq, "0.32"));
}

#[test]
fn split_of_a_tilde_takes_the_tilde() {
    // Arrange & Act
    let split = Op::split("~1.2");

    // Assert
    assert_eq!(split, (Op::Tilde, "1.2"));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::cmp::Ordering;
use ripe4rust_core::values::pre_release::PreRelease;

#[test]
fn cmp_of_a_longer_list_with_the_same_start_is_greater() {
    // Arrange
    let shorter = PreRelease::new("alpha");

    // Act
    let ordering = PreRelease::new("alpha.1").cmp(&shorter);

    // Assert
    assert_eq!(ordering, Ordering::Greater);
}

#[test]
fn cmp_of_letters_is_alphabetical() {
    // Arrange
    let alpha = PreRelease::new("alpha");

    // Act
    let ordering = alpha.cmp(&PreRelease::new("beta"));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn cmp_of_none_is_after_any_pre_release() {
    // Arrange
    let none = PreRelease::new("");

    // Act
    let ordering = none.cmp(&PreRelease::new("rc.9"));

    // Assert
    assert_eq!(ordering, Ordering::Greater);
}

#[test]
fn cmp_of_numbers_is_numeric() {
    // Arrange
    let two = PreRelease::new("rc.2");

    // Act
    let ordering = two.cmp(&PreRelease::new("rc.10"));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn cmp_puts_a_number_before_letters() {
    // Arrange
    let number = PreRelease::new("1");

    // Act
    let ordering = number.cmp(&PreRelease::new("alpha"));

    // Assert
    assert_eq!(ordering, Ordering::Less);
}

#[test]
fn fmt_writes_the_identifiers() {
    // Arrange & Act
    let text = PreRelease::new("rc.1").to_string();

    // Assert
    assert_eq!(text, "rc.1");
}

#[test]
fn is_empty_of_none_is_true() {
    // Arrange & Act
    let empty = PreRelease::new("").is_empty();

    // Assert
    assert!(empty);
}

#[test]
fn is_well_formed_of_an_empty_identifier_is_false() {
    // Arrange & Act
    let well_formed = PreRelease::new("rc..1").is_well_formed();

    // Assert
    assert!(!well_formed);
}

#[test]
fn is_well_formed_of_dotted_identifiers_is_true() {
    // Arrange & Act
    let well_formed = PreRelease::new("rc.1").is_well_formed();

    // Assert
    assert!(well_formed);
}

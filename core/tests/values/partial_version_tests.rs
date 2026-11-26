// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::edge::Edge;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;

fn partial(text: &str) -> PartialVersion {
    PartialVersion::parse(text).expect("a partial version")
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn beyond_of_a_full_version_is_the_next_patch() {
    // Arrange & Act
    let beyond = partial("1.2.3").beyond();

    // Assert
    assert_eq!(beyond, Version::new(1, 2, 4));
}

#[test]
fn beyond_of_a_major_is_the_next_major() {
    // Arrange & Act
    let beyond = partial("1").beyond();

    // Assert
    assert_eq!(beyond, Version::new(2, 0, 0));
}

#[test]
fn beyond_of_a_minor_is_the_next_minor() {
    // Arrange & Act
    let beyond = partial("0.32").beyond();

    // Assert
    assert_eq!(beyond, Version::new(0, 33, 0));
}

#[test]
fn caret_ceiling_of_a_bare_zero_is_one() {
    // Arrange & Act
    let ceiling = partial("0").caret_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(1, 0, 0));
}

#[test]
fn caret_ceiling_of_a_major_above_zero_is_the_next_major() {
    // Arrange & Act
    let ceiling = partial("1.2.3").caret_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(2, 0, 0));
}

#[test]
fn caret_ceiling_of_a_zero_major_is_the_next_minor() {
    // Arrange & Act
    let ceiling = partial("0.13.1").caret_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(0, 14, 0));
}

#[test]
fn caret_ceiling_of_zero_zero_is_zero_one() {
    // Arrange & Act
    let ceiling = partial("0.0").caret_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(0, 1, 0));
}

#[test]
fn caret_ceiling_of_zero_zero_patch_is_the_next_patch() {
    // Arrange & Act
    let ceiling = partial("0.0.7").caret_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(0, 0, 8));
}

#[test]
fn floor_fills_the_missing_parts_with_zero() {
    // Arrange & Act
    let floor = partial("1.74").floor();

    // Assert
    assert_eq!(floor, Version::new(1, 74, 0));
}

#[test]
fn floor_keeps_the_pre_release() {
    // Arrange & Act
    let floor = partial("2.0.0-rc.1").floor();

    // Assert
    assert_eq!(floor, version("2.0.0-rc.1"));
}

#[test]
fn fmt_writes_the_parts_given() {
    // Arrange & Act
    let text = partial("1.74").to_string();

    // Assert
    assert_eq!(text, "1.74");
}

#[test]
fn is_full_of_a_minor_is_false() {
    // Arrange & Act
    let full = partial("1.2").is_full();

    // Assert
    assert!(!full);
}

#[test]
fn is_full_of_three_parts_is_true() {
    // Arrange & Act
    let full = partial("1.2.3").is_full();

    // Assert
    assert!(full);
}

#[test]
fn last_of_a_full_version_is_it_inclusive() {
    // Arrange & Act
    let last = partial("1.2.3").last();

    // Assert
    assert_eq!(last, Edge::Inclusive(Version::new(1, 2, 3)));
}

#[test]
fn last_of_a_minor_is_the_next_minor_exclusive() {
    // Arrange & Act
    let last = partial("1.2").last();

    // Assert
    assert_eq!(last, Edge::Exclusive(Version::new(1, 3, 0)));
}

#[test]
fn names_pre_release_of_another_release_is_false() {
    // Arrange & Act
    let names = partial("2.0.0-rc.1").names_pre_release_of(&version("2.0.1-rc.1"));

    // Assert
    assert!(!names);
}

#[test]
fn names_pre_release_of_its_own_release_is_true() {
    // Arrange & Act
    let names = partial("2.0.0-rc.1").names_pre_release_of(&version("2.0.0-rc.3"));

    // Assert
    assert!(names);
}

#[test]
fn names_pre_release_without_one_of_its_own_is_false() {
    // Arrange & Act
    let names = partial("2.0.0").names_pre_release_of(&version("2.0.0-rc.1"));

    // Assert
    assert!(!names);
}

#[test]
fn parse_of_a_letter_fails() {
    // Arrange & Act
    let parsed = PartialVersion::parse("1.a");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_a_pre_release_without_a_patch_fails() {
    // Arrange & Act
    let parsed = PartialVersion::parse("1.2-rc.1");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_four_parts_fails() {
    // Arrange & Act
    let parsed = PartialVersion::parse("1.2.3.4");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn tilde_ceiling_of_a_major_is_the_next_major() {
    // Arrange & Act
    let ceiling = partial("1").tilde_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(2, 0, 0));
}

#[test]
fn tilde_ceiling_of_a_minor_is_the_next_minor() {
    // Arrange & Act
    let ceiling = partial("1.2.3").tilde_ceiling();

    // Assert
    assert_eq!(ceiling, Version::new(1, 3, 0));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::parse_failure::ParseFailure;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;

fn requirement(text: &str) -> Requirement {
    Requirement::parse(text).expect("a requirement")
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn exact_of_a_single_exact_version_is_that_version() {
    // Arrange & Act
    let exact = requirement("=1.0.229").exact();

    // Assert
    assert_eq!(exact, Some(Version::new(1, 0, 229)));
}

#[test]
fn exact_of_two_comparators_is_none() {
    // Arrange & Act
    let exact = requirement("=1.0.229, <2").exact();

    // Assert
    assert_eq!(exact, None);
}

#[test]
fn fmt_writes_the_requirement_as_given() {
    // Arrange & Act
    let text = requirement(" >=0.32 ").to_string();

    // Assert
    assert_eq!(text, ">=0.32");
}

#[test]
fn matches_needs_every_comparator() {
    // Arrange
    let range = requirement(">=1.2.0, <1.5");

    // Act & Assert
    assert!(range.matches(&version("1.4.9")));
    assert!(!range.matches(&version("1.5.0")));
    assert!(!range.matches(&version("1.1.0")));
}

#[test]
fn matches_of_a_pre_release_a_comparator_names_is_true() {
    // Arrange & Act
    let matched = requirement(">=2.0.0-rc.1").matches(&version("2.0.0-rc.2"));

    // Assert
    assert!(matched);
}

#[test]
fn matches_of_a_pre_release_its_comparators_do_not_name_is_false() {
    // Arrange & Act
    let matched = requirement(">=1.0").matches(&version("2.0.0-rc.1"));

    // Assert
    assert!(!matched);
}

#[test]
fn parse_of_a_bad_comparator_fails_naming_the_whole_text() {
    // Arrange & Act
    let parsed = Requirement::parse(">=1.0, <two");

    // Assert
    assert_eq!(
        parsed,
        Err(ParseFailure {
            text: String::from(">=1.0, <two"),
            expected: "a version requirement",
        })
    );
}

#[test]
fn parse_of_an_empty_text_fails() {
    // Arrange & Act
    let parsed = Requirement::parse("  ");

    // Assert
    assert_eq!(
        parsed,
        Err(ParseFailure {
            text: String::from("  "),
            expected: "a version requirement",
        })
    );
}

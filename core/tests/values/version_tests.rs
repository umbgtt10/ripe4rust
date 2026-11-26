// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::parse_failure::ParseFailure;
use ripe4rust_core::values::pre_release::PreRelease;
use ripe4rust_core::values::version::Version;

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn cmp_orders_a_minor_numerically() {
    // Arrange
    let earlier = version("1.2.3");

    // Act
    let later = version("1.10.0");

    // Assert
    assert!(earlier < later);
}

#[test]
fn cmp_puts_a_pre_release_before_its_release() {
    // Arrange
    let pre_release = version("2.0.0-rc.1");

    // Act
    let release = version("2.0.0");

    // Assert
    assert!(pre_release < release);
    assert!(version("1.9.9") < pre_release);
}

#[test]
fn fmt_writes_the_pre_release_after_the_patch() {
    // Arrange & Act
    let text = version("1.2.3-alpha.1").to_string();

    // Assert
    assert_eq!(text, "1.2.3-alpha.1");
}

#[test]
fn is_pre_release_of_a_pre_release_is_true() {
    // Arrange & Act
    let pre_release = version("0.5.0-beta").is_pre_release();

    // Assert
    assert!(pre_release);
}

#[test]
fn is_pre_release_of_a_release_is_false() {
    // Arrange & Act
    let pre_release = version("0.5.0").is_pre_release();

    // Assert
    assert!(!pre_release);
}

#[test]
fn line_of_a_major_above_zero_is_its_major() {
    // Arrange & Act
    let line = version("2.0.119").line();

    // Assert
    assert_eq!(
        line,
        Line {
            major: 2,
            minor: None
        }
    );
}

#[test]
fn line_of_a_zero_major_is_its_minor() {
    // Arrange & Act
    let line = version("0.13.1").line();

    // Assert
    assert_eq!(
        line,
        Line {
            major: 0,
            minor: Some(13)
        }
    );
}

#[test]
fn new_builds_the_release_it_names() {
    // Arrange & Act
    let built = Version::new(4, 6, 7);

    // Assert
    assert_eq!(built, version("4.6.7"));
}

#[test]
fn parse_drops_build_metadata() {
    // Arrange & Act
    let parsed = version("1.0.104+build.5");

    // Assert
    assert_eq!(parsed, Version::new(1, 0, 104));
}

#[test]
fn parse_of_a_letter_fails_naming_the_text() {
    // Arrange & Act
    let parsed = Version::parse("1.x.0");

    // Assert
    assert_eq!(
        parsed,
        Err(ParseFailure {
            text: String::from("1.x.0"),
            expected: "a version",
        })
    );
}

#[test]
fn parse_of_an_empty_pre_release_fails() {
    // Arrange & Act
    let parsed = Version::parse("1.0.0-");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_two_parts_fails() {
    // Arrange & Act
    let parsed = Version::parse("1.2");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn release_drops_the_pre_release() {
    // Arrange & Act
    let release = version("3.0.0-rc.2").release();

    // Assert
    assert_eq!(release, Version::new(3, 0, 0));
}

#[test]
fn toolchain_line_is_its_minor() {
    // Arrange & Act
    let line = version("1.100.0").toolchain_line();

    // Assert
    assert_eq!(
        line,
        Line {
            major: 1,
            minor: Some(100)
        }
    );
}

#[test]
fn with_pre_release_marks_the_release_as_one() {
    // Arrange & Act
    let pre_release = Version::new(2, 0, 0).with_pre_release(PreRelease::new("rc.1"));

    // Assert
    assert_eq!(pre_release, version("2.0.0-rc.1"));
}

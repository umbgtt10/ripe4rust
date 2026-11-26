// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::comparator::Comparator;
use ripe4rust_core::values::version::Version;

fn comparator(text: &str) -> Comparator {
    Comparator::parse(text).expect("a comparator")
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn exact_of_a_lower_bound_is_none() {
    // Arrange & Act
    let exact = comparator(">=1.0.229").exact();

    // Assert
    assert_eq!(exact, None);
}

#[test]
fn exact_of_an_exact_full_version_is_that_version() {
    // Arrange & Act
    let exact = comparator("=1.0.229").exact();

    // Assert
    assert_eq!(exact, Some(Version::new(1, 0, 229)));
}

#[test]
fn exact_of_an_exact_partial_is_none() {
    // Arrange & Act
    let exact = comparator("=1.0").exact();

    // Assert
    assert_eq!(exact, None);
}

#[test]
fn matches_of_a_bare_version_is_a_caret() {
    // Arrange
    let bare = comparator("1.2.3");

    // Act & Assert
    assert!(bare.matches(&version("1.9.0")));
    assert!(!bare.matches(&version("2.0.0")));
    assert!(!bare.matches(&version("1.2.2")));
}

#[test]
fn matches_of_a_caret_on_a_zero_major_stays_in_the_minor() {
    // Arrange
    let caret = comparator("^0.13.1");

    // Act & Assert
    assert!(caret.matches(&version("0.13.9")));
    assert!(!caret.matches(&version("0.14.0")));
}

#[test]
fn matches_of_a_greater_full_version_excludes_it() {
    // Arrange
    let greater = comparator(">1.2.3");

    // Act & Assert
    assert!(!greater.matches(&version("1.2.3")));
    assert!(greater.matches(&version("1.2.4")));
}

#[test]
fn matches_of_a_greater_partial_starts_past_it() {
    // Arrange
    let greater = comparator(">1.2");

    // Act & Assert
    assert!(!greater.matches(&version("1.2.9")));
    assert!(greater.matches(&version("1.3.0")));
}

#[test]
fn matches_of_a_less_or_equal_full_version_admits_it() {
    // Arrange
    let at_most = comparator("<=1.2.3");

    // Act & Assert
    assert!(at_most.matches(&version("1.2.3")));
    assert!(!at_most.matches(&version("1.2.4")));
}

#[test]
fn matches_of_a_less_or_equal_partial_admits_its_line() {
    // Arrange
    let at_most = comparator("<=1.2");

    // Act & Assert
    assert!(at_most.matches(&version("1.2.9")));
    assert!(!at_most.matches(&version("1.3.0")));
}

#[test]
fn matches_of_a_less_partial_stops_before_it() {
    // Arrange
    let less = comparator("<2");

    // Act & Assert
    assert!(less.matches(&version("1.99.0")));
    assert!(!less.matches(&version("2.0.0")));
}

#[test]
fn matches_of_a_lower_bound_on_a_minor_admits_it() {
    // Arrange
    let lower = comparator(">=0.32");

    // Act & Assert
    assert!(lower.matches(&version("0.32.0")));
    assert!(lower.matches(&version("1.0.0")));
    assert!(!lower.matches(&version("0.31.9")));
}

#[test]
fn matches_of_a_star_admits_everything() {
    // Arrange
    let any = comparator("*");

    // Act & Assert
    assert!(any.matches(&version("0.0.1")));
    assert!(any.matches(&version("99.0.0")));
}

#[test]
fn matches_of_a_tilde_stays_in_the_minor() {
    // Arrange
    let tilde = comparator("~1.2.3");

    // Act & Assert
    assert!(tilde.matches(&version("1.2.9")));
    assert!(!tilde.matches(&version("1.3.0")));
}

#[test]
fn matches_of_a_wildcard_minor_stays_in_the_major() {
    // Arrange
    let wildcard = comparator("1.*");

    // Act & Assert
    assert!(wildcard.matches(&version("1.7.0")));
    assert!(!wildcard.matches(&version("2.0.0")));
}

#[test]
fn matches_of_an_exact_full_version_is_only_it() {
    // Arrange
    let exact = comparator("=1.0.229");

    // Act & Assert
    assert!(exact.matches(&version("1.0.229")));
    assert!(!exact.matches(&version("1.0.230")));
}

#[test]
fn matches_of_an_exact_partial_is_its_line() {
    // Arrange
    let exact = comparator("=1.2");

    // Act & Assert
    assert!(exact.matches(&version("1.2.5")));
    assert!(!exact.matches(&version("1.3.0")));
}

#[test]
fn names_pre_release_of_a_comparator_naming_it_is_true() {
    // Arrange & Act
    let names = comparator(">=2.0.0-rc.1").names_pre_release_of(&version("2.0.0-rc.2"));

    // Assert
    assert!(names);
}

#[test]
fn parse_of_a_wildcard_before_a_number_fails() {
    // Arrange & Act
    let parsed = Comparator::parse("1.*.3");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_an_operator_alone_fails() {
    // Arrange & Act
    let parsed = Comparator::parse(">=");

    // Assert
    assert!(parsed.is_err());
}

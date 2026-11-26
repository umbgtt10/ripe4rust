// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use ripe4rust_core::planning::choice::line_stats::LineStats;
use ripe4rust_core::planning::choice::maturity::Maturity;
use ripe4rust_core::values::line::Line;

fn maturity() -> Maturity {
    Maturity::new(date("2026-11-26"), 30, 3, 6)
}

fn stats(first: Option<&str>, last: Option<&str>, releases: u32) -> LineStats {
    LineStats {
        line: Line {
            major: 2,
            minor: None,
        },
        first_release: first.map(date),
        last_release: last.map(date),
        releases,
    }
}

#[test]
fn is_active_of_a_last_release_before_the_months_is_false() {
    // Arrange & Act
    let active = maturity().is_active(&stats(Some("2024-01-01"), Some("2026-05-25"), 9));

    // Assert
    assert!(!active);
}

#[test]
fn is_active_of_a_release_within_the_months_is_true() {
    // Arrange & Act
    let active = maturity().is_active(&stats(Some("2024-01-01"), Some("2026-05-26"), 9));

    // Assert
    assert!(active);
}

#[test]
fn is_active_of_an_undated_line_is_false() {
    // Arrange & Act
    let active = maturity().is_active(&stats(None, None, 9));

    // Assert
    assert!(!active);
}

#[test]
fn is_young_of_a_line_first_released_within_the_days_is_true() {
    // Arrange & Act
    let young = maturity().is_young(&stats(Some("2026-10-28"), Some("2026-11-20"), 5));

    // Assert
    assert!(young);
}

#[test]
fn is_young_of_a_line_with_too_few_releases_is_true() {
    // Arrange & Act
    let young = maturity().is_young(&stats(Some("2025-01-01"), Some("2025-02-01"), 2));

    // Assert
    assert!(young);
}

#[test]
fn is_young_of_an_old_line_of_enough_releases_is_false() {
    // Arrange & Act
    let young = maturity().is_young(&stats(Some("2026-10-27"), Some("2026-11-20"), 3));

    // Assert
    assert!(!young);
}

#[test]
fn is_young_of_an_undated_line_of_enough_releases_is_false() {
    // Arrange & Act
    let young = maturity().is_young(&stats(None, None, 3));

    // Assert
    assert!(!young);
}

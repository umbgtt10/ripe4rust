// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::needing;
use crate::support::releases::release;
use crate::support::releases::version;
use ripe4rust_core::planning::choice::breaking_choice::BreakingChoice;
use ripe4rust_core::planning::choice::breaking_target::BreakingTarget;
use ripe4rust_core::planning::choice::maturity::Maturity;
use ripe4rust_core::planning::choice::release_history::ReleaseHistory;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::version::Version;

fn major(number: u64) -> Line {
    Line {
        major: number,
        minor: None,
    }
}

fn maturity() -> Maturity {
    Maturity::new(date("2026-11-26"), 30, 3, 6)
}

#[test]
fn choose_of_a_crate_on_its_newest_line_is_none() {
    // Arrange
    let history = ReleaseHistory::new(
        vec![
            release("1.0.0", "2025-01-01"),
            release("1.1.0", "2025-02-01"),
        ],
        Version::line,
    );
    let maturity = maturity();
    let toolchain = version("1.99.1");

    // Act
    let target = BreakingChoice::new(&history, &maturity, &toolchain).choose(&version("1.0.0"));

    // Assert
    assert_eq!(target, None);
}

#[test]
fn choose_of_only_young_lines_takes_the_newest_and_says_it_is_young() {
    // Arrange
    let history = ReleaseHistory::new(
        vec![
            release("1.0.0", "2024-01-01"),
            release("2.0.0", "2026-11-01"),
            release("3.0.0", "2026-11-20"),
        ],
        Version::line,
    );
    let maturity = maturity();
    let toolchain = version("1.99.1");

    // Act
    let target = BreakingChoice::new(&history, &maturity, &toolchain).choose(&version("1.0.0"));

    // Assert
    assert_eq!(
        target,
        Some(BreakingTarget {
            to: version("3.0.0"),
            line: major(3),
            young: true,
            younger_lines: Vec::new(),
            lines_behind: 2,
        })
    );
}

#[test]
fn choose_takes_an_older_release_of_a_line_whose_newest_the_toolchain_cannot_build() {
    // Arrange
    let history = ReleaseHistory::new(
        vec![
            release("1.0.0", "2024-01-01"),
            release("2.0.0", "2025-01-01"),
            release("2.1.0", "2025-02-01"),
            release("2.2.0", "2025-03-01"),
            needing("2.3.0", "2025-04-01", "1.100"),
        ],
        Version::line,
    );
    let maturity = maturity();
    let toolchain = version("1.99.1");

    // Act
    let target = BreakingChoice::new(&history, &maturity, &toolchain).choose(&version("1.0.0"));

    // Assert
    assert_eq!(target.map(|target| target.to), Some(version("2.2.0")));
}

#[test]
fn choose_takes_the_newest_settled_line_and_names_a_younger_one_beyond_it() {
    // Arrange
    let history = ReleaseHistory::new(
        vec![
            release("1.0.0", "2024-01-01"),
            release("2.0.0", "2025-01-01"),
            release("2.0.1", "2025-02-01"),
            release("2.0.2", "2025-03-01"),
            release("3.0.0", "2026-11-20"),
        ],
        Version::line,
    );
    let maturity = maturity();
    let toolchain = version("1.99.1");

    // Act
    let target = BreakingChoice::new(&history, &maturity, &toolchain).choose(&version("1.0.0"));

    // Assert
    assert_eq!(
        target,
        Some(BreakingTarget {
            to: version("2.0.2"),
            line: major(2),
            young: false,
            younger_lines: vec![major(3)],
            lines_behind: 2,
        })
    );
}

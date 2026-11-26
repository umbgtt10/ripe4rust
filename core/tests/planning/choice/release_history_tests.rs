// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::needing;
use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::releases::yanked;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::planning::choice::line_stats::LineStats;
use ripe4rust_core::planning::choice::release_history::ReleaseHistory;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;

fn history() -> ReleaseHistory {
    ReleaseHistory::new(
        vec![
            release("1.0.0", "2025-01-10"),
            release("1.4.0", "2025-06-01"),
            yanked("1.5.0", "2025-07-01"),
            release("2.0.0-rc.1", "2025-08-01"),
            release("2.0.0", "2025-09-01"),
            needing("2.1.0", "2026-10-01", "1.100"),
            release("3.0.0", "2026-11-20"),
        ],
        Version::line,
    )
}

#[test]
fn candidates_drop_yanked_pre_releases_and_what_the_toolchain_cannot_build() {
    // Arrange
    let history = history();

    // Act
    let candidates = history.candidates(&version("1.99.1"));

    // Assert
    assert_eq!(
        candidates,
        vec![
            version("1.0.0"),
            version("1.4.0"),
            version("2.0.0"),
            version("3.0.0")
        ]
    );
}

#[test]
fn held_back_leaves_out_what_the_next_toolchain_cannot_build_either() {
    // Arrange
    let history = history();

    // Act
    let held = history.held_back(&version("1.99.1"), &version("1.4.0"), &version("1.99.2"));

    // Assert
    assert_eq!(held, Vec::<Version>::new());
}

#[test]
fn held_back_lists_newer_releases_the_toolchain_cannot_build_and_the_next_one_can() {
    // Arrange
    let history = history();

    // Act
    let held = history.held_back(&version("1.99.1"), &version("1.4.0"), &version("1.100.0"));

    // Assert
    assert_eq!(held, vec![version("2.1.0")]);
}

#[test]
fn is_yanked_of_a_yanked_release_is_true() {
    // Arrange & Act
    let withdrawn = history().is_yanked(&version("1.5.0"));

    // Assert
    assert!(withdrawn);
}

#[test]
fn is_yanked_of_an_unknown_version_is_false() {
    // Arrange & Act
    let withdrawn = history().is_yanked(&version("9.9.9"));

    // Assert
    assert!(!withdrawn);
}

#[test]
fn line_of_follows_the_scheme_given() {
    // Arrange
    let toolchain = ReleaseHistory::new(Vec::new(), Version::toolchain_line);

    // Act
    let line = toolchain.line_of(&version("1.100.1"));

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
fn line_stats_counts_the_line_s_releases_and_dates_its_first_and_last() {
    // Arrange & Act
    let stats = history().line_stats(Line {
        major: 1,
        minor: None,
    });

    // Assert
    assert_eq!(
        stats,
        LineStats {
            line: Line {
                major: 1,
                minor: None
            },
            first_release: Some(date("2025-01-10")),
            last_release: Some(date("2025-06-01")),
            releases: 2,
        }
    );
}

#[test]
fn line_stats_of_undated_releases_has_no_dates() {
    // Arrange
    let undated = Release {
        published: None,
        ..release("0.3.0", "2020-01-01")
    };
    let history = ReleaseHistory::new(vec![undated], Version::line);

    // Act
    let stats = history.line_stats(Line {
        major: 0,
        minor: Some(3),
    });

    // Assert
    assert_eq!(stats.first_release, None);
    assert_eq!(stats.releases, 1);
}

#[test]
fn newer_lines_are_the_candidate_lines_past_the_one_given() {
    // Arrange & Act
    let lines = history().newer_lines(
        Line {
            major: 1,
            minor: None,
        },
        &version("1.99.1"),
    );

    // Assert
    assert_eq!(
        lines,
        vec![
            Line {
                major: 2,
                minor: None
            },
            Line {
                major: 3,
                minor: None
            }
        ]
    );
}

#[test]
fn newest_in_a_line_is_its_newest_candidate() {
    // Arrange & Act
    let newest = history().newest_in(
        Line {
            major: 2,
            minor: None,
        },
        &version("1.100.0"),
    );

    // Assert
    assert_eq!(newest, Some(version("2.1.0")));
}

#[test]
fn newest_matching_a_requirement_skips_yanked_releases() {
    // Arrange
    let requirement = Requirement::parse(">=1.5").expect("a requirement");

    // Act
    let newest = history().newest_matching(&requirement);

    // Assert
    assert_eq!(newest, Some(version("3.0.0")));
}

#[test]
fn published_on_of_a_release_is_its_date() {
    // Arrange & Act
    let published = history().published_on(&version("3.0.0"));

    // Assert
    assert_eq!(published, Some(date("2026-11-20")));
}

#[test]
fn rust_version_of_a_release_is_what_it_declares() {
    // Arrange & Act
    let rust = history().rust_version_of(&version("2.1.0"));

    // Assert
    assert_eq!(
        rust.map(|rust| rust.to_string()),
        Some(String::from("1.100"))
    );
}

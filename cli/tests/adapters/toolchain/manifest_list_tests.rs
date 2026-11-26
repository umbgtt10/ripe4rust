// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::toolchain::manifest_list::ManifestList;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::version::Version;

const RECORDED: &str = include_str!("../../fixtures/workspace/toolchain/manifests.txt");

fn dated(releases: &[Release], version: &str) -> Option<Date> {
    releases
        .iter()
        .find(|release| release.version == Version::parse(version).expect("a version"))
        .and_then(|release| release.published)
}

fn day(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

fn internal(detail: &str) -> PortFailure {
    PortFailure::Internal {
        doing: String::from("reading the Rust distribution's list of manifests"),
        detail: String::from(detail),
    }
}

#[test]
fn releases_of_a_list_naming_no_stable_release_are_unreadable() {
    // Arrange & Act
    let releases = ManifestList.releases(
        "<html><body>Sign in to the network</body></html>\nstatic.rust-lang.org/dist/2026-11-26/channel-rust-nightly.toml\n",
    );

    // Assert
    assert_eq!(releases, Err(internal("the list names no stable release")));
}

#[test]
fn releases_of_a_release_listed_on_several_days_are_dated_by_the_last() {
    // Arrange & Act
    let releases = ManifestList.releases(RECORDED).expect("releases");

    // Assert
    assert_eq!(
        (
            dated(&releases, "1.8.0"),
            dated(&releases, "1.14.0"),
            dated(&releases, "1.49.0"),
        ),
        (
            Some(day("2016-04-14")),
            Some(day("2016-12-21")),
            Some(day("2020-12-31"))
        )
    );
}

#[test]
fn releases_of_a_stable_line_whose_day_is_no_date_are_unreadable() {
    // Arrange & Act
    let releases =
        ManifestList.releases("static.rust-lang.org/dist/2026-13-12/channel-rust-1.100.0.toml\n");

    // Assert
    assert_eq!(
        releases,
        Err(internal("`2026-13-12` is not a date, YYYY-MM-DD"))
    );
}

#[test]
fn releases_of_the_recorded_list_are_every_stable_release_with_its_day() {
    // Arrange & Act
    let releases = ManifestList.releases(RECORDED).expect("releases");

    // Assert
    assert_eq!(releases.len(), 135);
    assert_eq!(
        releases.last(),
        Some(&Release {
            version: Version::parse("1.100.0").expect("a version"),
            yanked: false,
            rust_version: None,
            published: Some(day("2026-11-12")),
        })
    );
    assert_eq!(
        (dated(&releases, "1.99.0"), dated(&releases, "1.99.1")),
        (Some(day("2026-10-01")), Some(day("2026-10-15")))
    );
}

#[test]
fn releases_pass_over_nightlies_betas_and_a_line_s_alias() {
    // Arrange & Act
    let releases = ManifestList.releases(
        "static.rust-lang.org/dist/2026-11-12/channel-rust-stable.toml\nstatic.rust-lang.org/dist/2026-11-12/channel-rust-1.100.toml\nstatic.rust-lang.org/dist/2026-11-12/channel-rust-1.100.0.toml\nstatic.rust-lang.org/dist/2026-11-13/channel-rust-1.101.0-beta.toml\nstatic.rust-lang.org/dist/2026-11-14/channel-rust-1.101.0-beta.2.toml\nstatic.rust-lang.org/dist/2026-11-14/channel-rust-beta.toml\nstatic.rust-lang.org/dist/2026-11-26/channel-rust-nightly.toml\n200",
    );

    // Assert
    assert_eq!(
        releases.map(|releases| {
            releases
                .into_iter()
                .map(|release| release.version.to_string())
                .collect::<Vec<String>>()
        }),
        Ok(vec![String::from("1.100.0")])
    );
}

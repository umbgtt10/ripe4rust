// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::registry::index_reader::IndexReader;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;

const ITOA: &str = include_str!("../../fixtures/workspace/registry/index/itoa.txt");

fn body(recorded: &str) -> &str {
    recorded
        .rsplit_once('\n')
        .map_or(recorded, |(body, _)| body)
}

fn internal(detail: &str) -> PortFailure {
    PortFailure::Internal {
        doing: String::from("reading the index's answer for `widget`"),
        detail: String::from(detail),
    }
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn releases_of_an_empty_answer_are_none() {
    // Arrange & Act
    let releases = IndexReader.releases("widget", "");

    // Assert
    assert_eq!(releases, Ok(Vec::new()));
}

#[test]
fn releases_of_an_entry_that_is_not_json_are_internal() {
    // Arrange & Act
    let releases = IndexReader.releases("widget", "{\"vers\":");

    // Assert
    assert!(
        matches!(&releases, Err(PortFailure::Internal { doing, .. }) if doing == "reading the index's answer for `widget`"),
        "{releases:?}"
    );
}

#[test]
fn releases_of_an_entry_with_a_pubtime_that_is_no_time_are_internal() {
    // Arrange & Act
    let releases = IndexReader.releases(
        "widget",
        r#"{"name":"widget","vers":"0.1.0","yanked":false,"pubtime":"soon"}"#,
    );

    // Assert
    assert_eq!(releases, Err(internal("`soon` is not a date, YYYY-MM-DD")));
}

#[test]
fn releases_of_an_entry_with_a_version_that_is_none_are_internal() {
    // Arrange & Act
    let releases =
        IndexReader.releases("widget", r#"{"name":"widget","vers":"one","yanked":false}"#);

    // Assert
    assert_eq!(releases, Err(internal("`one` is not a version")));
}

#[test]
fn releases_of_an_entry_with_no_pubtime_have_no_date() {
    // Arrange & Act
    let releases = IndexReader.releases(
        "widget",
        r#"{"name":"widget","vers":"0.1.0","yanked":false}"#,
    );

    // Assert
    assert_eq!(
        releases,
        Ok(vec![Release {
            version: version("0.1.0"),
            yanked: false,
            rust_version: None,
            published: None,
        }])
    );
}

#[test]
fn releases_of_the_recorded_itoa_index_are_every_version_with_its_date() {
    // Arrange & Act
    let releases = IndexReader.releases("itoa", body(ITOA)).expect("releases");

    // Assert
    assert_eq!(releases.len(), 38);
    assert_eq!(
        releases[0],
        Release {
            version: version("0.1.0"),
            yanked: false,
            rust_version: None,
            published: Some(Date::parse("2016-06-25").expect("a date")),
        }
    );
    let first_major = releases
        .iter()
        .find(|release| release.version == version("1.0.0"))
        .expect("1.0.0");
    assert_eq!(
        first_major.rust_version,
        Some(PartialVersion::parse("1.36").expect("a rust-version"))
    );
}

#[test]
fn releases_of_the_recorded_smallvec_index_say_which_are_yanked() {
    // Arrange & Act
    let releases = IndexReader
        .releases(
            "smallvec",
            body(include_str!(
                "../../fixtures/workspace/registry/index/smallvec.txt"
            )),
        )
        .expect("releases");

    // Assert
    let yanked = |text: &str| {
        releases
            .iter()
            .find(|release| release.version == version(text))
            .map(|release| release.yanked)
    };
    assert_eq!(
        (yanked("0.3.2"), yanked("1.6.0")),
        (Some(true), Some(false))
    );
}

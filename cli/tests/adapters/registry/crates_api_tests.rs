// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::registry::crates_api::CratesApi;
use cargo_ripe4rust::adapters::registry::curl::Curl;
use cargo_ripe4rust::adapters::registry::day_cache::DayCache;
use cargo_ripe4rust::adapters::registry::lookup::Lookup;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::repositories::Repositories;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

fn recorded() -> ReplayRunner {
    ReplayRunner::new()
        .reply(
            &curl("https://crates.io/api/v1/crates/itoa?include="),
            printed(
                include_str!("../../fixtures/workspace/registry/api/itoa.txt"),
                "",
            ),
        )
        .reply(
            &curl("https://crates.io/api/v1/crates/smallvec?include="),
            printed(
                include_str!("../../fixtures/workspace/registry/api/smallvec.txt"),
                "",
            ),
        )
        .reply(
            &curl("https://crates.io/api/v1/crates/gone?include="),
            printed(include_str!("../../fixtures/registry/api-missing.txt"), ""),
        )
        .reply(
            &curl("https://crates.io/api/v1/crates/busy?include="),
            printed("{}\n429", ""),
        )
}

fn today() -> Date {
    Date::parse("2026-11-26").expect("a date")
}

#[test]
fn repository_answered_with_another_status_is_unreachable() {
    // Arrange
    let (runner, files, pause) = (recorded(), MemoryFiles::new(), CountingPause::new());
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let repository = CratesApi::new(&curl, &cache, &pause).repository("busy");

    // Assert
    assert_eq!(
        repository,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from(
                "the API answered 429 for `https://crates.io/api/v1/crates/busy?include=`"
            ),
        })
    );
}

#[test]
fn repository_of_a_cached_crate_asks_nothing_and_waits_for_nothing() {
    // Arrange
    let (runner, pause) = (ReplayRunner::new(), CountingPause::new());
    let files = MemoryFiles::new()
        .with(
            "/cache/2026-11-26/api/itoa",
            r#"{"crate":{"repository":"https://example.org/itoa"}}"#,
        )
        .with("/cache/2026-11-26/api/smallvec", "");
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );
    let api = CratesApi::new(&curl, &cache, &pause);

    // Act
    let repositories = [api.repository("itoa"), api.repository("smallvec")];

    // Assert
    assert_eq!(
        repositories,
        [Ok(Some(String::from("https://example.org/itoa"))), Ok(None)]
    );
    assert_eq!(runner.commands(), Vec::<String>::new());
    assert_eq!(*pause.pauses.borrow(), Vec::<u64>::new());
}

#[test]
fn repository_of_a_crate_the_api_does_not_know_is_none_cached_as_such() {
    // Arrange
    let (runner, files, pause) = (recorded(), MemoryFiles::new(), CountingPause::new());
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let repository = CratesApi::new(&curl, &cache, &pause).repository("gone");

    // Assert
    assert_eq!(repository, Ok(None));
    assert_eq!(
        files.contents("/cache/2026-11-26/api/gone"),
        Some(String::new())
    );
}

#[test]
fn repository_of_two_crates_asks_the_api_a_second_apart() {
    // Arrange
    let (runner, files, pause) = (recorded(), MemoryFiles::new(), CountingPause::new());
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );
    let api = CratesApi::new(&curl, &cache, &pause);

    // Act
    let repositories = [api.repository("itoa"), api.repository("smallvec")];

    // Assert
    assert_eq!(
        repositories,
        [
            Ok(Some(String::from("https://github.com/dtolnay/itoa"))),
            Ok(Some(String::from("https://github.com/smallvec/smallvec"))),
        ]
    );
    assert_eq!(*pause.pauses.borrow(), vec![1]);
    assert!(files.contents("/cache/2026-11-26/api/itoa").is_some());
}

#[test]
fn repository_refreshed_of_a_cached_crate_asks_the_api_again() {
    // Arrange
    let (runner, pause) = (recorded(), CountingPause::new());
    let files = MemoryFiles::new().with("/cache/2026-11-26/api/itoa", "");
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Refreshed,
    );

    // Act
    let repository = CratesApi::new(&curl, &cache, &pause).repository("itoa");

    // Assert
    assert_eq!(
        repository,
        Ok(Some(String::from("https://github.com/dtolnay/itoa")))
    );
    let (body, _) = include_str!("../../fixtures/workspace/registry/api/itoa.txt")
        .rsplit_once('\n')
        .expect("a status");
    assert_eq!(
        files.contents("/cache/2026-11-26/api/itoa"),
        Some(String::from(body))
    );
}

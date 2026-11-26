// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::registry::cached_answers::CachedAnswers;
use cargo_ripe4rust::adapters::registry::curl::Curl;
use cargo_ripe4rust::adapters::registry::day_cache::DayCache;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const URL: &str = "https://index.crates.io/it/oa/itoa";

fn today() -> Date {
    Date::parse("2026-11-26").expect("a date")
}

#[test]
fn cached_of_a_day_holding_the_answer_is_it() {
    // Arrange
    let runner = ReplayRunner::new();
    let files = MemoryFiles::new().with("/cache/2026-11-26/index/itoa", "an answer");
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let cached = CachedAnswers::new(&curl, &cache).cached("index", "itoa");

    // Assert
    assert_eq!(cached, Ok(Some(String::from("an answer"))));
}

#[test]
fn fetched_of_a_200_is_its_body_kept_for_the_day() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed("an answer\n200", ""));
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let fetched = CachedAnswers::new(&curl, &cache).fetched("index", "itoa", URL, "the index");

    // Assert
    assert_eq!(fetched, Ok(String::from("an answer")));
    assert_eq!(
        files.contents("/cache/2026-11-26/index/itoa"),
        Some(String::from("an answer"))
    );
}

#[test]
fn fetched_of_a_404_is_nothing_kept_for_the_day() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed("not here\n404", ""));
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let fetched = CachedAnswers::new(&curl, &cache).fetched("index", "itoa", URL, "the index");

    // Assert
    assert_eq!(fetched, Ok(String::new()));
    assert_eq!(
        files.contents("/cache/2026-11-26/index/itoa"),
        Some(String::new())
    );
}

#[test]
fn fetched_of_another_status_is_unreachable_naming_who_answered() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed("\n500", ""));
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let fetched = CachedAnswers::new(&curl, &cache).fetched("index", "itoa", URL, "the index");

    // Assert
    assert_eq!(
        fetched,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from("the index answered 500 for `https://index.crates.io/it/oa/itoa`"),
        })
    );
    assert_eq!(files.contents("/cache/2026-11-26/index/itoa"), None);
}

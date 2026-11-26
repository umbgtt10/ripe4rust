// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::registry::curl::Curl;
use cargo_ripe4rust::adapters::registry::day_cache::DayCache;
use cargo_ripe4rust::adapters::registry::sparse_index::SparseIndex;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::index::Index;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const ITOA: &str = include_str!("../../fixtures/workspace/registry/index/itoa.txt");
const URL: &str = "https://index.crates.io/it/oa/itoa";

fn today() -> Date {
    Date::parse("2026-11-26").expect("a date")
}

#[test]
fn read_on_is_the_cache_s_day() {
    // Arrange
    let (runner, files) = (ReplayRunner::new(), MemoryFiles::new());
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let day = SparseIndex::new(&curl, &cache).read_on();

    // Assert
    assert_eq!(day, today());
}

#[test]
fn releases_answered_with_another_status_are_unreachable() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed("busy\n503", ""));
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let releases = SparseIndex::new(&curl, &cache).releases("itoa");

    // Assert
    assert_eq!(
        releases,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from("the index answered 503 for `https://index.crates.io/it/oa/itoa`"),
        })
    );
}

#[test]
fn releases_of_a_cached_crate_read_the_cache_alone() {
    // Arrange
    let runner = ReplayRunner::new();
    let files = MemoryFiles::new().with(
        "/cache/2026-11-26/index/itoa",
        r#"{"name":"itoa","vers":"1.0.19","yanked":false}"#,
    );
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let releases = SparseIndex::new(&curl, &cache).releases("itoa");

    // Assert
    assert_eq!(releases.map(|releases| releases.len()), Ok(1));
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn releases_of_a_crate_fetch_its_index_file_and_cache_it_for_the_day() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed(ITOA, ""));
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let releases = SparseIndex::new(&curl, &cache).releases("itoa");

    // Assert
    assert_eq!(releases.map(|releases| releases.len()), Ok(38));
    let (body, _) = ITOA.rsplit_once('\n').expect("a status");
    assert_eq!(
        files.contents("/cache/2026-11-26/index/itoa"),
        Some(String::from(body))
    );
}

#[test]
fn releases_of_a_crate_the_index_does_not_hold_are_none_cached_as_such() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        printed(
            include_str!("../../fixtures/registry/index-missing.txt"),
            "",
        ),
    );
    let files = MemoryFiles::new();
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);

    // Act
    let releases = SparseIndex::new(&curl, &cache).releases("itoa");

    // Assert
    assert_eq!(releases, Ok(Vec::new()));
    assert_eq!(
        files.contents("/cache/2026-11-26/index/itoa"),
        Some(String::new())
    );
}

#[test]
fn url_of_a_name_follows_the_index_s_layout_by_its_length() {
    // Arrange
    let (runner, files) = (ReplayRunner::new(), MemoryFiles::new());
    let curl = Curl::new(&runner, PathBuf::from("/work/ws"));
    let cache = DayCache::new(&files, Some(PathBuf::from("/cache")), today(), false);
    let index = SparseIndex::new(&curl, &cache);

    // Act
    let urls = ["a", "ab", "abc", "Smallvec"].map(|name| index.url(name));

    // Assert
    assert_eq!(
        urls,
        [
            "https://index.crates.io/1/a",
            "https://index.crates.io/2/ab",
            "https://index.crates.io/3/a/abc",
            "https://index.crates.io/sm/al/smallvec",
        ]
        .map(String::from)
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::registry::curl::Curl;
use cargo_ripe4rust::adapters::registry::day_cache::DayCache;
use cargo_ripe4rust::adapters::registry::lookup::Lookup;
use cargo_ripe4rust::adapters::toolchain::channel_manifests::ChannelManifests;
use cargo_ripe4rust::system::traits::release_source::ReleaseSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

const CACHED: &str = "/cache/2026-11-26/toolchain/manifests.txt";
const RECORDED: &str = include_str!("../../fixtures/workspace/toolchain/manifests.txt");
const URL: &str = "https://static.rust-lang.org/manifests.txt";

fn body(recorded: &str) -> &str {
    recorded
        .rsplit_once('\n')
        .map_or(recorded, |(body, _)| body)
}

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

fn today() -> Date {
    Date::parse("2026-11-26").expect("a date")
}

fn unreachable(detail: &str) -> PortFailure {
    PortFailure::ToolchainUnreachable {
        detail: String::from(detail),
    }
}

#[test]
fn releases_answered_with_another_status_are_unreachable_and_not_kept() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        printed(
            include_str!("../../fixtures/toolchain/manifests-missing.txt"),
            "",
        ),
    );
    let files = MemoryFiles::new();
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache).releases();

    // Assert
    assert_eq!(
        releases,
        Err(unreachable(
            "the Rust distribution answered 404 for `https://static.rust-lang.org/manifests.txt`"
        ))
    );
    assert_eq!(files.contents(CACHED), None);
}

#[test]
fn releases_cached_for_the_day_are_read_from_the_cache_alone() {
    // Arrange
    let runner = ReplayRunner::new();
    let files = MemoryFiles::new().with(
        CACHED,
        "static.rust-lang.org/dist/2026-11-12/channel-rust-1.100.0.toml\n",
    );
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache).releases();

    // Assert
    assert_eq!(releases.map(|releases| releases.len()), Ok(1));
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn releases_fetch_the_distribution_s_list_and_keep_it_for_the_day() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed(RECORDED, ""));
    let files = MemoryFiles::new();
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache)
        .releases()
        .expect("releases");

    // Assert
    assert_eq!(
        releases.last().map(|release| &release.version),
        Some(&Version::parse("1.100.0").expect("a version"))
    );
    assert_eq!(files.contents(CACHED).as_deref(), Some(body(RECORDED)));
}

#[test]
fn releases_of_a_list_that_does_not_read_are_not_kept() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        printed("<html><body>Sign in to the network</body></html>\n200", ""),
    );
    let files = MemoryFiles::new();
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache).releases();

    // Assert
    assert!(
        matches!(releases, Err(PortFailure::Internal { .. })),
        "{releases:?}"
    );
    assert_eq!(files.contents(CACHED), None);
}

#[test]
fn releases_offline_with_nothing_cached_are_unreachable() {
    // Arrange
    let runner = ReplayRunner::new();
    let files = MemoryFiles::new();
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Offline,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache).releases();

    // Assert
    assert_eq!(
        releases,
        Err(unreachable(
            "`--offline`, and nothing is cached on or before 2026-11-26"
        ))
    );
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn releases_out_of_curl_s_reach_are_unreachable_with_what_it_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        failed(
            6,
            include_str!("../../fixtures/toolchain/manifests-unreachable.txt"),
        ),
    );
    let files = MemoryFiles::new();
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Cached,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache).releases();

    // Assert
    assert_eq!(
        releases,
        Err(unreachable(
            "curl: (6) Could not resolve host: static.rust-lang.invalid"
        ))
    );
}

#[test]
fn releases_refreshed_fetch_the_list_again_over_the_day_s_copy() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed(RECORDED, ""));
    let files = MemoryFiles::new().with(
        CACHED,
        "static.rust-lang.org/dist/2026-10-30/channel-rust-1.99.1.toml\n",
    );
    let http = Curl::new(&runner, root());
    let cache = DayCache::new(
        &files,
        Some(PathBuf::from("/cache")),
        today(),
        Lookup::Refreshed,
    );

    // Act
    let releases = ChannelManifests::new(&http, &cache)
        .releases()
        .expect("releases");

    // Assert
    assert_eq!(
        releases.last().map(|release| &release.version),
        Some(&Version::parse("1.100.0").expect("a version"))
    );
    assert_eq!(files.contents(CACHED).as_deref(), Some(body(RECORDED)));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use cargo_ripe4rust::adapters::registry::day_cache::DayCache;
use cargo_ripe4rust::system::traits::answer_cache::AnswerCache;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

fn cached() -> MemoryFiles {
    MemoryFiles::new()
        .with(
            "/cache/ripe4rust/2026-11-20/index/smallvec",
            "an earlier answer",
        )
        .with(
            "/cache/ripe4rust/2026-11-24/index/smallvec",
            "the latest answer",
        )
        .with("/cache/ripe4rust/2026-11-24/api/smallvec", "")
        .with(
            "/cache/ripe4rust/2026-11-30/index/smallvec",
            "a later answer",
        )
        .with("/cache/ripe4rust/notes/index/smallvec", "not a day")
}

fn date(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

fn root() -> PathBuf {
    PathBuf::from("/cache/ripe4rust")
}

#[test]
fn read_of_a_day_with_no_answer_is_none_to_fetch() {
    // Arrange
    let files = cached();

    // Act
    let read =
        DayCache::new(&files, Some(root()), date("2026-11-26"), false).read("index", "smallvec");

    // Assert
    assert_eq!(read, Ok(None));
}

#[test]
fn read_of_an_unreadable_answer_is_an_unusable_cache() {
    // Arrange
    let files = cached().refusing("/cache/ripe4rust/2026-11-24/index/smallvec");

    // Act
    let read =
        DayCache::new(&files, Some(root()), date("2026-11-24"), false).read("index", "smallvec");

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::RegistryCacheUnusable {
            detail: String::from(
                "`/cache/ripe4rust/2026-11-24/index/smallvec` could not be read: permission denied"
            ),
        })
    );
}

#[test]
fn read_offline_of_an_answer_the_day_lacks_is_unreachable() {
    // Arrange
    let files = cached();

    // Act
    let read =
        DayCache::new(&files, Some(root()), date("2026-11-26"), true).read("index", "either");

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from(
                "`--offline`, and the cache of 2026-11-24 holds no index answer for `either`"
            ),
        })
    );
}

#[test]
fn read_offline_takes_the_latest_day_up_to_today() {
    // Arrange
    let files = cached();

    // Act
    let read =
        DayCache::new(&files, Some(root()), date("2026-11-26"), true).read("index", "smallvec");

    // Assert
    assert_eq!(read, Ok(Some(String::from("the latest answer"))));
}

#[test]
fn read_offline_with_nothing_cached_is_unreachable() {
    // Arrange
    let files = MemoryFiles::new();

    // Act
    let read =
        DayCache::new(&files, Some(root()), date("2026-11-26"), true).read("index", "smallvec");

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from("`--offline`, and nothing is cached on or before 2026-11-26"),
        })
    );
}

#[test]
fn read_on_offline_is_the_day_read() {
    // Arrange
    let files = cached();

    // Act
    let day = DayCache::new(&files, Some(root()), date("2026-11-26"), true).read_on();

    // Assert
    assert_eq!(day, date("2026-11-24"));
}

#[test]
fn read_on_online_is_today() {
    // Arrange
    let files = cached();

    // Act
    let day = DayCache::new(&files, Some(root()), date("2026-11-26"), false).read_on();

    // Assert
    assert_eq!(day, date("2026-11-26"));
}

#[test]
fn write_keeps_the_answer_under_today() {
    // Arrange
    let files = MemoryFiles::new();
    let cache = DayCache::new(&files, Some(root()), date("2026-11-26"), false);

    // Act
    let written = cache.write("api", "smallvec", "{}");

    // Assert
    assert_eq!(written, Ok(()));
    assert_eq!(
        files.contents("/cache/ripe4rust/2026-11-26/api/smallvec"),
        Some(String::from("{}"))
    );
    assert_eq!(cache.read("api", "smallvec"), Ok(Some(String::from("{}"))));
}

#[test]
fn write_refused_is_an_unusable_cache() {
    // Arrange
    let files = MemoryFiles::new().refusing("/cache/ripe4rust/2026-11-26/api/smallvec");

    // Act
    let written = DayCache::new(&files, Some(root()), date("2026-11-26"), false)
        .write("api", "smallvec", "{}");

    // Assert
    assert_eq!(
        written,
        Err(PortFailure::RegistryCacheUnusable {
            detail: String::from(
                "`/cache/ripe4rust/2026-11-26/api/smallvec` could not be written: read-only file system"
            ),
        })
    );
}

#[test]
fn write_with_no_cache_directory_is_an_unusable_cache() {
    // Arrange
    let files = MemoryFiles::new();

    // Act
    let written =
        DayCache::new(&files, None, date("2026-11-26"), false).write("api", "smallvec", "{}");

    // Assert
    assert_eq!(
        written,
        Err(PortFailure::RegistryCacheUnusable {
            detail: String::from(
                "there is no cache directory: neither `XDG_CACHE_HOME` nor `HOME` is set, nor `LOCALAPPDATA` on Windows"
            ),
        })
    );
}

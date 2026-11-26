// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::registry::cache_root::CacheRoot;
use std::path::PathBuf;

#[test]
fn path_elsewhere_falls_back_to_the_home_s_cache() {
    // Arrange & Act
    let path = CacheRoot::new(None, None, Some(String::from("/home/someone")), false).path();

    // Assert
    assert_eq!(path, Some(PathBuf::from("/home/someone/.cache/ripe4rust")));
}

#[test]
fn path_elsewhere_follows_xdg_cache_home() {
    // Arrange & Act
    let path = CacheRoot::new(
        Some(String::from("/var/cache/me")),
        None,
        Some(String::from("/home/someone")),
        false,
    )
    .path();

    // Assert
    assert_eq!(path, Some(PathBuf::from("/var/cache/me/ripe4rust")));
}

#[test]
fn path_elsewhere_skips_an_empty_xdg_cache_home() {
    // Arrange & Act
    let path = CacheRoot::new(
        Some(String::new()),
        None,
        Some(String::from("/home/someone")),
        false,
    )
    .path();

    // Assert
    assert_eq!(path, Some(PathBuf::from("/home/someone/.cache/ripe4rust")));
}

#[test]
fn path_on_windows_is_under_local_app_data() {
    // Arrange & Act
    let path = CacheRoot::new(
        Some(String::from("/ignored")),
        Some(String::from("C:/Users/someone/AppData/Local")),
        None,
        true,
    )
    .path();

    // Assert
    assert_eq!(
        path,
        Some(PathBuf::from("C:/Users/someone/AppData/Local/ripe4rust"))
    );
}

#[test]
fn path_with_no_place_to_cache_is_none() {
    // Arrange & Act
    let path = CacheRoot::new(None, None, None, false).path();

    // Assert
    assert_eq!(path, None);
}

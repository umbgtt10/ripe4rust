// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::root_names::RootNames;
use std::path::Path;
use std::path::PathBuf;

#[test]
fn name_of_a_file_outside_the_root_is_its_own_path() {
    // Arrange
    let root = PathBuf::from("/work/ws");

    // Act
    let name = RootNames::new(&root).name(Path::new("/elsewhere/build.rs"));

    // Assert
    assert_eq!(name, "/elsewhere/build.rs");
}

#[test]
fn name_of_a_file_under_the_root_is_its_path_from_it_with_slashes() {
    // Arrange
    let root = PathBuf::from("/work/ws");

    // Act
    let name = RootNames::new(&root).name(&root.join("app-core").join("src").join("lib.rs"));

    // Assert
    assert_eq!(name, "app-core/src/lib.rs");
}

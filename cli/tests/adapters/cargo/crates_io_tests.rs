// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::crates_io::CratesIo;

#[test]
fn holds_a_package_from_the_index_by_git_or_sparse() {
    // Arrange & Act
    let held = [
        "registry+https://github.com/rust-lang/crates.io-index",
        "sparse+https://index.crates.io/",
    ]
    .map(|source| CratesIo.holds(Some(source)));

    // Assert
    assert_eq!(held, [true, true]);
}

#[test]
fn holds_no_path_package_and_no_other_registry_s() {
    // Arrange & Act
    let held =
        [None, Some("registry+https://example.org/index")].map(|source| CratesIo.holds(source));

    // Assert
    assert_eq!(held, [false, false]);
}

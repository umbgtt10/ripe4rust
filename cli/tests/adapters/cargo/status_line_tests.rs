// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::status_line::StatusLine;

#[test]
fn knows_cargo_s_messages_and_statuses_that_move_nothing() {
    // Arrange & Act
    let known = [
        "    Updating crates.io index",
        "    Updating `example` index",
        "    Updating git repository `https://example.org/widget`",
        "    Updating git submodule `https://example.org/widget-sys`",
        "     Locking 1 package to highest Rust 1.85 compatible version",
        "     Locking 34 packages to latest compatible versions",
        "    Blocking waiting for file lock on package cache",
        "   Unchanged widget v1.4.0 (available: v2.0.0)",
        "note: pass `--verbose` to see 3 unchanged dependencies behind latest",
        "warning: not updating lockfile due to --dry-run",
    ]
    .map(|line| StatusLine.knows(line));

    // Assert
    assert_eq!(known, [true; 10]);
}

#[test]
fn knows_no_line_it_was_not_told_of() {
    // Arrange & Act
    let known = [
        "    Frobbing widget v1.4.0",
        "     Locking the door",
        "    Updating the world",
        "package:   /work/ws/app/Cargo.toml",
    ]
    .map(|line| StatusLine.knows(line));

    // Assert
    assert_eq!(known, [false; 4]);
}

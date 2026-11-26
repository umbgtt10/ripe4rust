// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::moves::added;
use crate::support::moves::removed;
use crate::support::moves::updated;
use cargo_ripe4rust::adapters::cargo::move_line::MoveLine;

#[test]
fn read_of_a_downgrade_moves_the_crate_back() {
    // Arrange & Act
    let read = MoveLine.read(" Downgrading memchr v2.7.0 -> v2.5.0");

    // Assert
    assert_eq!(read, Some(updated("memchr", "2.7.0", "2.5.0")));
}

#[test]
fn read_of_a_removal_with_build_metadata_drops_it() {
    // Arrange & Act
    let read = MoveLine.read("    Removing wasi v0.10.0+wasi-snapshot-preview1");

    // Assert
    assert_eq!(read, Some(removed("wasi", "0.10.0")));
}

#[test]
fn read_of_a_status_line_is_no_move() {
    // Arrange & Act
    let read = MoveLine.read("    Updating crates.io index");

    // Assert
    assert_eq!(read, None);
}

#[test]
fn read_of_an_addition_has_no_from() {
    // Arrange & Act
    let read = MoveLine.read("      Adding itoa v0.4.8 (available: v1.0.19)");

    // Assert
    assert_eq!(read, Some(added("itoa", "0.4.8")));
}

#[test]
fn read_of_an_update_needing_a_newer_rust_is_a_move() {
    // Arrange & Act
    let read = MoveLine.read("    Updating widget v1.4.0 -> v1.4.1 (requires Rust 1.90)");

    // Assert
    assert_eq!(read, Some(updated("widget", "1.4.0", "1.4.1")));
}

#[test]
fn read_of_an_update_with_a_newer_line_available_is_a_move() {
    // Arrange & Act
    let read = MoveLine.read("    Updating itertools v0.10.0 -> v0.10.5 (available: v0.15.0)");

    // Assert
    assert_eq!(read, Some(updated("itertools", "0.10.0", "0.10.5")));
}

#[test]
fn read_of_an_update_with_a_note_it_does_not_know_is_no_move() {
    // Arrange & Act
    let read = MoveLine.read("    Updating memchr v2.5.0 -> v2.8.3 (yanked)");

    // Assert
    assert_eq!(read, None);
}

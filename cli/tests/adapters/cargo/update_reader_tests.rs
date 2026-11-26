// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::moves::added;
use crate::support::moves::removed;
use crate::support::moves::updated;
use cargo_ripe4rust::adapters::cargo::update_reader::UpdateReader;
use ripe4rust_core::inputs::port_failure::PortFailure;

#[test]
fn moves_of_a_line_it_does_not_know_stop_the_plan() {
    // Arrange
    let text = "    Updating crates.io index\n    Frobbing widget v1.4.0 -> v1.4.1\n";

    // Act
    let moves = UpdateReader.moves(text);

    // Assert
    assert_eq!(
        moves,
        Err(PortFailure::CargoUpdateUnrecognised {
            line: String::from("Frobbing widget v1.4.0 -> v1.4.1"),
        })
    );
}

#[test]
fn moves_of_a_recorded_downgrade_move_the_crate_back() {
    // Arrange & Act
    let moves = UpdateReader.moves(include_str!("../../fixtures/cargo-update/downgrade.txt"));

    // Assert
    assert_eq!(moves, Ok(vec![updated("memchr", "2.7.0", "2.5.0")]));
}

#[test]
fn moves_of_a_recorded_update_moving_nothing_are_none() {
    // Arrange & Act
    let moves = UpdateReader.moves(include_str!("../../fixtures/cargo-update/nothing.txt"));

    // Assert
    assert_eq!(moves, Ok(Vec::new()));
}

#[test]
fn moves_of_a_recorded_workspace_update_are_every_update_in_order() {
    // Arrange & Act
    let moves = UpdateReader.moves(include_str!("../../fixtures/workspace/cargo/update.txt"));

    // Assert
    assert_eq!(
        moves,
        Ok(vec![
            updated("either", "1.5.0", "1.19.0"),
            updated("itertools", "0.10.0", "0.10.5"),
            updated("itoa", "0.4.7", "0.4.8"),
            updated("memchr", "2.5.0", "2.8.3"),
            updated("smallvec", "1.6.0", "1.16.6"),
        ])
    );
}

#[test]
fn moves_of_recorded_additions_and_removals_keep_each() {
    // Arrange & Act
    let moves = UpdateReader
        .moves(include_str!("../../fixtures/cargo-update/moves-chrono.txt"))
        .expect("moves");

    // Assert
    assert_eq!(moves.len(), 39);
    assert_eq!(moves[0], added("android_system_properties", "0.1.6"));
    assert_eq!(moves[4], updated("chrono", "0.4.19", "0.4.45"));
    assert_eq!(moves[14], removed("num-integer", "0.1.47"));
    assert!(moves.contains(&added("syn", "2.0.119")));
    assert!(moves.contains(&added("syn", "3.0.10")));
    assert!(moves.contains(&removed("wasi", "0.10.0")));
}

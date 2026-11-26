// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::surveyed::at;
use crate::support::surveyed::written;
use ripe4rust_core::planning::round::line_change::LineChange;
use ripe4rust_core::planning::round::line_changes::LineChanges;

#[test]
fn changes_of_a_line_added_and_one_gone_have_no_counterpart() {
    // Arrange
    let mut current = lines();
    current.remove(2);
    current.push(written(at("app/Cargo.toml", 4), "tiny", "1"));
    let changes = LineChanges::new(&lines(), &current);

    // Act
    let found = changes.changes();

    // Assert
    assert_eq!(
        found,
        vec![
            LineChange::Gone {
                then: written(at("Cargo.toml", 22), "gadget", "0.13"),
            },
            LineChange::New {
                now: written(at("app/Cargo.toml", 4), "tiny", "1"),
            },
        ]
    );
}

#[test]
fn changes_of_a_value_rewritten_are_the_line_then_and_now() {
    // Arrange
    let changes = LineChanges::new(&lines(), &lines_with(&[(21, "1.5")]));

    // Act
    let found = changes.changes();

    // Assert
    assert_eq!(
        found,
        vec![LineChange::Rewritten {
            then: written(at("Cargo.toml", 21), "widget", "1.4"),
            now: written(at("Cargo.toml", 21), "widget", "1.5"),
        }]
    );
}

#[test]
fn changes_of_lines_as_written_are_none() {
    // Arrange
    let changes = LineChanges::new(&lines(), &lines());

    // Act
    let found = changes.changes();

    // Assert
    assert_eq!(found, Vec::new());
}

#[test]
fn changes_of_lines_moved_down_the_file_are_none() {
    // Arrange
    let shifted = lines()
        .into_iter()
        .map(|mut line| {
            line.at.line += 3;
            line
        })
        .collect::<Vec<_>>();
    let changes = LineChanges::new(&lines(), &shifted);

    // Act
    let found = changes.changes();

    // Assert
    assert_eq!(found, Vec::new());
}

#[test]
fn made_of_an_edit_is_whether_its_line_now_reads_its_value() {
    // Arrange
    let changes = LineChanges::new(&lines(), &lines_with(&[(22, "0.14.0")]));

    // Act
    let made = [
        written(at("Cargo.toml", 22), "gadget", "0.14.0"),
        written(at("Cargo.toml", 21), "widget", "2.0.1"),
        written(at("Cargo.toml", 40), "gizmo", "3.0.0"),
    ]
    .map(|edit| changes.made(&edit));

    // Assert
    assert_eq!(made, [true, false, false]);
}

#[test]
fn now_of_a_line_is_the_one_standing_for_it_wherever_it_moved() {
    // Arrange
    let mut current = lines();
    current[1].at.line = 24;
    let changes = LineChanges::new(&lines(), &current);

    // Act
    let now = changes.now(&at("Cargo.toml", 21), "widget");

    // Assert
    assert_eq!(now, Some(written(at("Cargo.toml", 24), "widget", "1.4")));
}

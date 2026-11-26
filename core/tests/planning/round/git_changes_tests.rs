// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::SOURCE;
use crate::support::recorded::git_at;
use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::planning::round::git_changes::GitChanges;

fn moved(from: &str, to: &str) -> GitSource {
    GitSource {
        source: String::from(SOURCE),
        from: String::from(from),
        to: String::from(to),
    }
}

#[test]
fn applied_of_a_source_at_the_rev_a_unit_takes_it_to_is_true() {
    // Arrange
    let changes = GitChanges::new(&git_at("9625c3d2"), &git_at("ce129c85"));

    // Act
    let applied = [
        changes.applied(&moved("9625c3d2", "ce129c85")),
        changes.applied(&moved("9625c3d2", "deadbeef")),
    ];

    // Assert
    assert_eq!(applied, [true, false]);
}

#[test]
fn rev_of_a_source_is_the_rev_the_lock_holds_it_at_now() {
    // Arrange
    let changes = GitChanges::new(&git_at("9625c3d2"), &git_at("ce129c85"));

    // Act
    let revs = [changes.rev(SOURCE), changes.rev("https://example.org/gone")];

    // Assert
    assert_eq!(revs, [Some(String::from("ce129c85")), None]);
}

#[test]
fn unexplained_are_the_sources_moved_that_no_move_applied_accounts_for() {
    // Arrange
    let changes = GitChanges::new(&git_at("9625c3d2"), &git_at("deadbeef"));

    // Act
    let unexplained = [
        changes.unexplained(&[]),
        changes.unexplained(&[moved("9625c3d2", "deadbeef")]),
    ];

    // Assert
    assert_eq!(
        unexplained,
        [vec![moved("9625c3d2", "deadbeef")], Vec::new()]
    );
}

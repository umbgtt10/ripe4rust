// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::moves::git;
use cargo_ripe4rust::adapters::cargo::git_move_line::GitMoveLine;

#[test]
fn read_of_a_git_crate_added_or_removed_names_the_one_rev() {
    // Arrange & Act
    let read = [
        GitMoveLine.read(
            "Adding gitextra v0.1.0 (https://github.com/example/upstream?branch=main#b52bc079)",
        ),
        GitMoveLine.read(
            "Removing gitmacros v0.1.0 (https://github.com/example/upstream?branch=main#9625c3d2)",
        ),
    ];

    // Assert
    assert_eq!(
        read,
        [
            Some(git("gitextra", "0.1.0", None, Some("b52bc079"))),
            Some(git("gitmacros", "0.1.0", Some("9625c3d2"), None)),
        ]
    );
}

#[test]
fn read_of_a_git_source_moved_names_the_crate_its_source_and_both_revs() {
    // Arrange & Act
    let read = [
        GitMoveLine.read("Updating gitcore v0.1.0 (https://github.com/example/upstream?branch=main#9625c3d2) -> #ce129c85"),
        GitMoveLine.read("Downgrading gitcore v0.2.0 (https://github.com/example/upstream?branch=main#9625c3d2) -> #ce129c85"),
    ];

    // Assert
    assert_eq!(
        read,
        [
            Some(git("gitcore", "0.1.0", Some("9625c3d2"), Some("ce129c85"))),
            Some(git("gitcore", "0.2.0", Some("9625c3d2"), Some("ce129c85"))),
        ]
    );
}

#[test]
fn read_of_a_registry_move_or_a_line_it_does_not_know_is_none() {
    // Arrange & Act
    let read = [
        GitMoveLine.read("Updating itoa v1.0.10 -> v1.0.19"),
        GitMoveLine.read("Updating gitcore v0.1.0 (https://github.com/example/upstream?branch=main#9625c3d2) -> v0.2.0"),
        GitMoveLine.read("Updating gitcore v0.1.0 (https://github.com/example/upstream?branch=main) -> #ce129c85"),
        GitMoveLine.read("Updating gitcore v0.1.0 (https://github.com/example/upstream?branch=main#9625c3d2) -> #ce129c85 (latest: v0.3.0)"),
        GitMoveLine.read("Frobbing gitcore v0.1.0 (https://github.com/example/upstream?branch=main#9625c3d2)"),
    ];

    // Assert
    assert_eq!(read, [None, None, None, None, None]);
}

#[test]
fn read_of_a_source_with_no_branch_keeps_it_as_written() {
    // Arrange & Act
    let read = GitMoveLine.read(
        "Updating gitcore v0.1.0 (https://github.com/example/upstream#9625c3d2) -> #ce129c85",
    );

    // Assert
    assert_eq!(
        read.map(|found| found.source),
        Some(String::from("https://github.com/example/upstream"))
    );
}

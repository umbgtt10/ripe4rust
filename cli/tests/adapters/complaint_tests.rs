// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::complaint::Complaint;
use cargo_ripe4rust::system::model::command_output::CommandOutput;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;

fn ended(status: Option<i32>, stderr: &str) -> CommandOutput {
    CommandOutput {
        status,
        stdout: String::new(),
        stderr: String::from(stderr),
    }
}

#[test]
fn line_of_a_message_over_several_lines_is_one_with_single_spaces() {
    // Arrange & Act
    let line = Complaint
        .line("failed to fetch crates:     Updating crates.io index\nerror: cannot update\n");

    // Assert
    assert_eq!(
        line,
        "failed to fetch crates: Updating crates.io index error: cannot update"
    );
}

#[test]
fn of_a_cargo_error_is_the_error_and_what_follows_it_on_one_line() {
    // Arrange
    let output = ended(
        Some(101),
        "    Updating crates.io index\nerror: failed to select a version for `widget`.\n    required by package `app v0.1.0`\n\n",
    );

    // Act
    let complaint = Complaint.of(&output);

    // Assert
    assert_eq!(
        complaint,
        "failed to select a version for `widget`. required by package `app v0.1.0`"
    );
}

#[test]
fn of_a_recorded_failure_is_cargo_s_error() {
    // Arrange
    let output = ended(
        Some(101),
        include_str!("../fixtures/cargo/metadata-nowhere.txt"),
    );

    // Act
    let complaint = Complaint.of(&output);

    // Assert
    assert_eq!(
        complaint,
        "could not find `Cargo.toml` in `/work/nowhere` or any parent directory"
    );
}

#[test]
fn of_a_signal_with_nothing_said_names_the_signal() {
    // Arrange & Act
    let complaint = Complaint.of(&ended(None, ""));

    // Assert
    assert_eq!(complaint, "it was ended by a signal");
}

#[test]
fn of_a_status_with_nothing_said_names_the_status() {
    // Arrange & Act
    let complaint = Complaint.of(&ended(Some(3), "\n"));

    // Assert
    assert_eq!(complaint, "it exited with status 3, saying nothing");
}

#[test]
fn of_words_with_no_error_line_is_every_line() {
    // Arrange & Act
    let complaint = Complaint.of(&ended(Some(1), "something broke\n  badly\n"));

    // Assert
    assert_eq!(complaint, "something broke badly");
}

#[test]
fn unlaunched_of_a_missing_program_says_it_is_not_on_the_path() {
    // Arrange & Act
    let complaint = Complaint.unlaunched(&LaunchFailure::Missing {
        program: String::from("curl"),
    });

    // Assert
    assert_eq!(complaint, "`curl` is not on the PATH");
}

#[test]
fn unlaunched_of_a_refused_program_says_what_the_system_said() {
    // Arrange & Act
    let complaint = Complaint.unlaunched(&LaunchFailure::Failed {
        program: String::from("cargo"),
        detail: String::from("permission denied"),
    });

    // Assert
    assert_eq!(complaint, "`cargo` could not be started: permission denied");
}

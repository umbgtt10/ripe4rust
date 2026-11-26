// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::Command;
use std::process::Output;

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-ripe4rust"))
        .args(args)
        .output()
        .expect("the binary under test runs")
}

#[test]
fn main_of_a_command_not_built_yet_exits_two() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn main_of_a_command_not_built_yet_prints_its_diagnostic_on_stdout() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "error[command-unavailable] `guide` is not in this release of ripe4rust yet. Do: use `cargo ripe4rust plan`, `check` or `hold`, the commands this release runs; `guide` comes with a later release\n"
    );
}

#[test]
fn main_of_a_command_not_built_yet_writes_nothing_to_stderr() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn main_of_an_unknown_command_in_json_prints_the_failure_document() {
    // Arrange & Act
    let output = run(&["ripe4rust", "frobnicate", "--format", "json"]);

    // Assert
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("\"code\": \"usage-invalid\""),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

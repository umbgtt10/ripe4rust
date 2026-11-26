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

#[test]
fn main_of_guide_exits_zero() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn main_of_guide_prints_the_guide_on_stdout() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert!(
        String::from_utf8_lossy(&output.stdout).starts_with(&format!(
            "ripe4rust {}: the guide\n",
            env!("CARGO_PKG_VERSION")
        )),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn main_of_guide_writes_nothing_to_stderr() {
    // Arrange & Act
    let output = run(&["ripe4rust", "guide"]);

    // Assert
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

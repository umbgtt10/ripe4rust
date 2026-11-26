// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::Command;
use std::process::Output;

const DESIGN: &str = "https://github.com/umbgtt10/ripe4rust/blob/main/docs/DESIGN.md";

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-ripe4rust"))
        .args(args)
        .output()
        .expect("the binary under test runs")
}

#[test]
fn main_at_the_design_stage_exits_two() {
    // Arrange & Act
    let output = run(&["ripe4rust"]);

    // Assert
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn main_at_the_design_stage_says_so_naming_the_outcomes_and_the_design() {
    // Arrange & Act
    let output = run(&["ripe4rust"]);

    // Assert
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        format!(
            "cargo-ripe4rust {}: design stage, no plan can be produced yet. Every upgrade will be ripe, ask or ripening; the design is at {DESIGN}\n",
            env!("CARGO_PKG_VERSION")
        )
    );
}

#[test]
fn main_at_the_design_stage_writes_nothing_to_stdout() {
    // Arrange & Act
    let output = run(&["ripe4rust"]);

    // Assert
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

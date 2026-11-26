// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::cargo::cargo_command::CargoCommand;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| String::from(*word)).collect()
}

#[test]
fn run_of_a_failing_command_is_the_named_failure_with_what_cargo_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &["cargo", "update", "--dry-run"],
        failed(101, "error: the lock file needs updating\n"),
    );

    // Act
    let run = CargoCommand::new(&runner, PathBuf::from("/work/ws"))
        .run(args(&["update", "--dry-run"]), |detail| {
            PortFailure::CargoUpdateFailed { detail }
        });

    // Assert
    assert_eq!(
        run,
        Err(PortFailure::CargoUpdateFailed {
            detail: String::from("the lock file needs updating"),
        })
    );
}

#[test]
fn run_of_a_succeeding_command_is_its_output_from_the_workspace() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &["cargo", "locate-project"],
        printed("/work/ws/Cargo.toml\n", ""),
    );

    // Act
    let run = CargoCommand::new(&runner, PathBuf::from("/work/ws"))
        .run(args(&["locate-project"]), |detail| {
            PortFailure::CargoMetadataFailed { detail }
        });

    // Assert
    assert_eq!(
        run.map(|output| output.stdout),
        Ok(String::from("/work/ws/Cargo.toml\n"))
    );
    assert_eq!(runner.calls.borrow()[0].dir, PathBuf::from("/work/ws"));
}

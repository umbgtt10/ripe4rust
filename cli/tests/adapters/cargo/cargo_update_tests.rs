// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::moves::updated;
use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::cargo::cargo_update::CargoUpdate;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::lock_moves::LockMoves;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

const WORKSPACE: [&str; 3] = ["cargo", "update", "--dry-run"];

fn of_itertools(offline: bool) -> Vec<&'static str> {
    let mut command = vec!["cargo", "update", "--dry-run"];
    if offline {
        command.push("--offline");
    }
    command.extend(["--recursive", "-p", "itertools@0.10.0"]);
    command
}

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn of_crate_asks_cargo_for_the_crate_at_its_version_and_what_it_pulls_in() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &of_itertools(false),
        printed(
            "",
            include_str!("../../fixtures/workspace/cargo/update-itertools.txt"),
        ),
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), false).of_crate("itertools", &version("0.10.0"));

    // Assert
    assert_eq!(
        moves,
        Ok(vec![
            updated("either", "1.5.0", "1.19.0"),
            updated("itertools", "0.10.0", "0.10.5"),
        ])
    );
    assert_eq!(runner.calls.borrow()[0].dir, root());
}

#[test]
fn of_crate_of_a_crate_cargo_does_not_know_says_what_cargo_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &of_itertools(false),
        failed(
            101,
            include_str!("../../fixtures/cargo-update/unknown-spec.txt"),
        ),
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), false).of_crate("itertools", &version("0.10.0"));

    // Assert
    assert_eq!(
        moves,
        Err(PortFailure::CargoUpdateFailed {
            detail: String::from(
                "package ID specification `nosuch@1.0.0` did not match any packages"
            ),
        })
    );
}

#[test]
fn of_crate_offline_asks_cargo_to_stay_offline() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &of_itertools(true),
        printed(
            "",
            include_str!("../../fixtures/workspace/cargo/update-itertools.txt"),
        ),
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), true).of_crate("itertools", &version("0.10.0"));

    // Assert
    assert_eq!(moves.map(|moves| moves.len()), Ok(2));
}

#[test]
fn of_source_asks_cargo_for_the_git_crate_at_its_version_alone() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &["cargo", "update", "--dry-run", "-p", "gitcore@0.1.0"],
        printed("", include_str!("../../fixtures/cargo-update/git-rev.txt")),
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), false).of_source("gitcore", &version("0.1.0"));

    // Assert
    assert_eq!(
        moves.map(|moves| (moves.registry.len(), moves.git.len())),
        Ok((1, 2))
    );
}

#[test]
fn of_workspace_asks_cargo_for_every_move() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &WORKSPACE,
        printed(
            "",
            include_str!("../../fixtures/workspace/cargo/update.txt"),
        ),
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), false).of_workspace();

    // Assert
    assert_eq!(
        moves.map(|moves| (moves.registry.len(), moves.git.len())),
        Ok((5, 0))
    );
    assert_eq!(
        runner.commands(),
        vec![String::from("cargo update --dry-run")]
    );
}

#[test]
fn of_workspace_without_cargo_says_it_could_not_start() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &WORKSPACE,
        LaunchFailure::Missing {
            program: String::from("cargo"),
        },
    );

    // Act
    let moves = CargoUpdate::new(&runner, root(), false).of_workspace();

    // Assert
    assert_eq!(
        moves,
        Err(PortFailure::CargoUpdateFailed {
            detail: String::from("`cargo` is not on the PATH"),
        })
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::cargo::deny::cargo_deny::CargoDeny;
use cargo_ripe4rust::system::model::command_output::CommandOutput;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::advisories::Advisories;
use std::path::PathBuf;

fn command(offline: bool) -> Vec<&'static str> {
    let mut command = vec!["cargo", "deny", "--format", "json", "--locked"];
    if offline {
        command.push("--offline");
    }
    command.extend([
        "--workspace",
        "check",
        "--audit-compatible-output",
        "advisories",
    ]);
    command
}

fn ended(status: i32, stdout: &str, stderr: &str) -> CommandOutput {
    CommandOutput {
        status: Some(status),
        stdout: String::from(stdout),
        stderr: String::from(stderr),
    }
}

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

#[test]
fn advisories_of_a_config_cargo_deny_cannot_read_fail_with_its_error() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &command(false),
        ended(
            1,
            "",
            include_str!("../../../fixtures/cargo-deny/broken.jsonl"),
        ),
    );

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(
        advisories,
        Err(PortFailure::CargoDenyFailed {
            detail: String::from(
                "failed to parse config from '/work/ws/deny.toml': expected a right bracket, found a newline"
            ),
        })
    );
}

#[test]
fn advisories_of_a_crash_say_what_cargo_deny_said() {
    // Arrange
    let runner =
        ReplayRunner::new().reply(&command(false), failed(101, "thread 'main' panicked\n"));

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(
        advisories,
        Err(PortFailure::CargoDenyFailed {
            detail: String::from("thread 'main' panicked"),
        })
    );
}

#[test]
fn advisories_of_an_ignored_advisory_leave_it_out() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &command(false),
        ended(
            1,
            include_str!("../../../fixtures/cargo-deny/ignored.json"),
            include_str!("../../../fixtures/cargo-deny/ignored.jsonl"),
        ),
    );

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(
        advisories.map(|found| found
            .into_iter()
            .map(|advisory| advisory.id)
            .collect::<Vec<_>>()),
        Ok(vec![String::from("RUSTSEC-2021-0139")])
    );
}

#[test]
fn advisories_of_the_recorded_workspace_name_smallvec_s() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &command(false),
        ended(
            1,
            include_str!("../../../fixtures/workspace/cargo-deny/audit.json"),
            include_str!("../../../fixtures/workspace/cargo-deny/log.jsonl"),
        ),
    );

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(
        advisories.map(|found| found
            .into_iter()
            .map(|advisory| (advisory.id, advisory.name))
            .collect::<Vec<_>>()),
        Ok(vec![(
            String::from("RUSTSEC-2021-0003"),
            String::from("smallvec")
        )])
    );
    assert_eq!(runner.calls.borrow()[0].dir, root());
}

#[test]
fn advisories_offline_ask_cargo_deny_to_stay_offline() {
    // Arrange
    let runner = ReplayRunner::new().reply(&command(true), ended(0, "", ""));

    // Act
    let advisories = CargoDeny::new(&runner, root(), true).advisories();

    // Assert
    assert_eq!(advisories, Ok(Vec::new()));
}

#[test]
fn advisories_without_cargo_deny_are_missing() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &command(false),
        failed(
            101,
            include_str!("../../../fixtures/cargo-deny/missing.txt"),
        ),
    );

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(advisories, Err(PortFailure::CargoDenyMissing));
}

#[test]
fn advisories_without_cargo_fail_saying_so() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &command(false),
        LaunchFailure::Missing {
            program: String::from("cargo"),
        },
    );

    // Act
    let advisories = CargoDeny::new(&runner, root(), false).advisories();

    // Assert
    assert_eq!(
        advisories,
        Err(PortFailure::CargoDenyFailed {
            detail: String::from("`cargo` is not on the PATH"),
        })
    );
}

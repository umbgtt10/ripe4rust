// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::toolchain::rustc_toolchain::RustcToolchain;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::toolchain::Toolchain;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

const TARGETS: [&str; 3] = ["rustc", "--print", "target-list"];
const VERSION: [&str; 2] = ["rustc", "--version"];

fn recorded() -> ReplayRunner {
    ReplayRunner::new()
        .reply(
            &VERSION,
            printed(
                include_str!("../../fixtures/workspace/rustc/version.txt"),
                "",
            ),
        )
        .reply(
            &TARGETS,
            printed(
                include_str!("../../fixtures/workspace/rustc/target-list.txt"),
                "",
            ),
        )
}

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

#[test]
fn releases_are_not_read_yet() {
    // Arrange
    let runner = ReplayRunner::new();

    // Act
    let releases = RustcToolchain::new(&runner, root()).releases();

    // Assert
    assert_eq!(releases, Ok(None));
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn rustc_of_a_version_line_it_cannot_read_is_internal() {
    // Arrange
    let runner = ReplayRunner::new().reply(&VERSION, printed("rustc\n", ""));

    // Act
    let rustc = RustcToolchain::new(&runner, root()).rustc();

    // Assert
    assert_eq!(
        rustc,
        Err(PortFailure::Internal {
            doing: String::from("reading what `rustc --version` printed"),
            detail: String::from("`rustc` names no version"),
        })
    );
}

#[test]
fn rustc_of_the_recorded_toolchain_is_its_release() {
    // Arrange
    let runner = recorded();

    // Act
    let rustc = RustcToolchain::new(&runner, root()).rustc();

    // Assert
    assert_eq!(rustc, Ok(Version::parse("1.99.1").expect("a version")));
    assert_eq!(runner.calls.borrow()[0].dir, root());
}

#[test]
fn rustc_without_rustc_is_unreachable() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &VERSION,
        LaunchFailure::Missing {
            program: String::from("rustc"),
        },
    );

    // Act
    let rustc = RustcToolchain::new(&runner, root()).rustc();

    // Assert
    assert_eq!(
        rustc,
        Err(PortFailure::ToolchainUnreachable {
            detail: String::from("`rustc` is not on the PATH"),
        })
    );
}

#[test]
fn targets_of_a_failing_rustc_are_unreachable_with_what_it_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &TARGETS,
        failed(
            1,
            "error: toolchain '1.99.1-x86_64-unknown-linux-gnu' is not installed\n",
        ),
    );

    // Act
    let targets = RustcToolchain::new(&runner, root()).targets();

    // Assert
    assert_eq!(
        targets,
        Err(PortFailure::ToolchainUnreachable {
            detail: String::from("toolchain '1.99.1-x86_64-unknown-linux-gnu' is not installed"),
        })
    );
}

#[test]
fn targets_of_the_recorded_toolchain_are_every_target_it_knows() {
    // Arrange
    let runner = recorded();

    // Act
    let targets = RustcToolchain::new(&runner, root())
        .targets()
        .expect("targets");

    // Assert
    assert!(targets.contains(&String::from("thumbv7em-none-eabihf")));
    assert!(targets.contains(&String::from("x86_64-unknown-linux-gnu")));
    assert_eq!(
        targets.len(),
        include_str!("../../fixtures/workspace/rustc/target-list.txt")
            .lines()
            .count()
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::cargo::cargo_metadata::CargoMetadata;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::member::Member;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::traits::workspace_metadata::WorkspaceMetadata;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

const METADATA: [&str; 5] = ["cargo", "metadata", "--format-version", "1", "--locked"];

fn recorded() -> ReplayRunner {
    ReplayRunner::new()
        .reply(
            &METADATA,
            printed(
                include_str!("../../fixtures/workspace/cargo/metadata.json"),
                "",
            ),
        )
        .reply(
            &[
                "cargo",
                "metadata",
                "--format-version",
                "1",
                "--locked",
                "--filter-platform",
                "thumbv7em-none-eabihf",
            ],
            printed(
                include_str!("../../fixtures/workspace/cargo/metadata-thumbv7em-none-eabihf.json"),
                "",
            ),
        )
}

fn resolved(name: &str, version: &str, requirement: &str) -> ResolvedDependency {
    ResolvedDependency {
        name: String::from(name),
        version: Version::parse(version).expect("a version"),
        requirement: Requirement::parse(requirement).expect("a requirement"),
    }
}

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

#[test]
fn reached_by_a_target_is_every_registry_crate_its_build_resolves() {
    // Arrange
    let runner = recorded();

    // Act
    let reached = CargoMetadata::new(&runner, root(), false).reached_by("thumbv7em-none-eabihf");

    // Assert
    assert_eq!(
        reached,
        Ok(vec![
            String::from("either"),
            String::from("itertools"),
            String::from("smallvec"),
        ])
    );
}

#[test]
fn reached_by_a_target_without_a_lock_is_lock_outdated() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &[
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--filter-platform",
            "thumbv7em-none-eabihf",
        ],
        failed(
            101,
            include_str!("../../fixtures/cargo/metadata-lock-missing.txt"),
        ),
    );

    // Act
    let reached = CargoMetadata::new(&runner, root(), false).reached_by("thumbv7em-none-eabihf");

    // Assert
    assert_eq!(
        reached,
        Err(PortFailure::LockOutdated {
            detail: String::from(
                "cannot create the lock file /work/ws/Cargo.lock because --locked was passed to prevent this"
            ),
        })
    );
}

#[test]
fn read_asks_cargo_for_the_locked_graph() {
    // Arrange
    let runner = recorded();

    // Act
    let _ = CargoMetadata::new(&runner, root(), false).read();

    // Assert
    assert_eq!(
        runner.commands(),
        vec![String::from("cargo metadata --format-version 1 --locked")]
    );
    assert_eq!(runner.calls.borrow()[0].dir, root());
}

#[test]
fn read_of_a_failing_cargo_metadata_says_what_cargo_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &METADATA,
        failed(
            101,
            include_str!("../../fixtures/cargo/metadata-nowhere.txt"),
        ),
    );

    // Act
    let read = CargoMetadata::new(&runner, root(), false).read();

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::CargoMetadataFailed {
            detail: String::from(
                "could not find `Cargo.toml` in `/work/nowhere` or any parent directory"
            ),
        })
    );
}

#[test]
fn read_of_a_lock_out_of_step_with_the_manifests_is_lock_outdated() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &METADATA,
        failed(
            101,
            include_str!("../../fixtures/cargo/metadata-lock-outdated.txt"),
        ),
    );

    // Act
    let read = CargoMetadata::new(&runner, root(), false).read();

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::LockOutdated {
            detail: String::from(
                "cannot update the lock file /work/ws/Cargo.lock because --locked was passed to prevent this"
            ),
        })
    );
}

#[test]
fn read_of_output_that_is_not_metadata_is_internal() {
    // Arrange
    let runner = ReplayRunner::new().reply(&METADATA, printed("{\"packages\": 3}", ""));

    // Act
    let read = CargoMetadata::new(&runner, root(), false).read();

    // Assert
    assert!(
        matches!(&read, Err(PortFailure::Internal { doing, .. }) if doing == "reading what `cargo metadata` printed"),
        "{read:?}"
    );
}

#[test]
fn read_of_the_recorded_workspace_locks_its_registry_packages() {
    // Arrange
    let runner = recorded();

    // Act
    let workspace = CargoMetadata::new(&runner, root(), false)
        .read()
        .expect("a workspace");

    // Assert
    let package =
        |name: &str, version: &str, dependencies: Vec<ResolvedDependency>| LockedPackage {
            name: String::from(name),
            version: Version::parse(version).expect("a version"),
            dependencies,
        };
    assert_eq!(
        workspace.lock,
        vec![
            package("either", "1.5.0", Vec::new()),
            package(
                "itertools",
                "0.10.0",
                vec![resolved("either", "1.5.0", "^1.0")]
            ),
            package("itoa", "0.4.7", Vec::new()),
            package("memchr", "2.5.0", Vec::new()),
            package("smallvec", "1.6.0", Vec::new()),
        ]
    );
}

#[test]
fn read_of_the_recorded_workspace_names_its_members_and_their_dependencies() {
    // Arrange
    let runner = recorded();

    // Act
    let workspace = CargoMetadata::new(&runner, root(), false)
        .read()
        .expect("a workspace");

    // Assert
    let rust_version = Some(PartialVersion::parse("1.85").expect("a rust-version"));
    assert_eq!(
        workspace.members,
        vec![
            Member {
                name: String::from("app"),
                rust_version: rust_version.clone(),
                dependencies: vec![
                    resolved("itertools", "0.10.0", "^0.10"),
                    resolved("memchr", "2.5.0", "^2.5"),
                    resolved("smallvec", "1.6.0", "^1.6.0"),
                ],
            },
            Member {
                name: String::from("app-core"),
                rust_version,
                dependencies: vec![resolved("itoa", "0.4.7", "^0.4")],
            },
        ]
    );
}

#[test]
fn read_offline_asks_cargo_to_stay_offline() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &[
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--offline",
        ],
        printed(
            include_str!("../../fixtures/workspace/cargo/metadata.json"),
            "",
        ),
    );

    // Act
    let read = CargoMetadata::new(&runner, root(), true).read();

    // Assert
    assert!(read.is_ok(), "{read:?}");
}

#[test]
fn read_without_cargo_says_it_could_not_start() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &METADATA,
        LaunchFailure::Missing {
            program: String::from("cargo"),
        },
    );

    // Act
    let read = CargoMetadata::new(&runner, root(), false).read();

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::CargoMetadataFailed {
            detail: String::from("`cargo` is not on the PATH"),
        })
    );
}

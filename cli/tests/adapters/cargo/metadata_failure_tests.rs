// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::metadata_failure::MetadataFailure;
use ripe4rust_core::inputs::port_failure::PortFailure;

const HELP: &str = "help: to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.";
const REFUSED: &str = "because --locked was passed to prevent this";

#[test]
fn of_any_other_failure_is_cargo_metadata_failed() {
    // Arrange & Act
    let failure = MetadataFailure.of(String::from(
        "could not find `Cargo.toml` in `/work/nowhere` or any parent directory",
    ));

    // Assert
    assert_eq!(
        failure,
        PortFailure::CargoMetadataFailed {
            detail: String::from(
                "could not find `Cargo.toml` in `/work/nowhere` or any parent directory"
            ),
        }
    );
}

#[test]
fn of_cargo_refusing_to_create_the_lock_is_lock_missing_without_cargo_s_help() {
    // Arrange & Act
    let failure = MetadataFailure.of(format!(
        "cannot create the lock file /work/ws/Cargo.lock {REFUSED} {HELP}"
    ));

    // Assert
    assert_eq!(
        failure,
        PortFailure::LockMissing {
            detail: format!("cannot create the lock file /work/ws/Cargo.lock {REFUSED}"),
        }
    );
}

#[test]
fn of_cargo_refusing_to_update_the_lock_is_lock_outdated_without_cargo_s_help() {
    // Arrange & Act
    let failure = MetadataFailure.of(format!(
        "cannot update the lock file /work/ws/Cargo.lock {REFUSED} {HELP}"
    ));

    // Assert
    assert_eq!(
        failure,
        PortFailure::LockOutdated {
            detail: format!("cannot update the lock file /work/ws/Cargo.lock {REFUSED}"),
        }
    );
}

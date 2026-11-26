// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::member::Member;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::planning::units::direct_dependencies::DirectDependencies;
use ripe4rust_core::planning::units::direct_dependency::DirectDependency;
use ripe4rust_core::values::requirement::Requirement;

fn locked(name: &str, at: &str) -> LockedPackage {
    LockedPackage {
        name: String::from(name),
        version: version(at),
        dependencies: Vec::new(),
    }
}

fn member(name: &str, dependencies: Vec<ResolvedDependency>) -> Member {
    Member {
        name: String::from(name),
        rust_version: None,
        dependencies,
    }
}

fn on(name: &str, at: &str) -> ResolvedDependency {
    ResolvedDependency {
        name: String::from(name),
        version: version(at),
        requirement: Requirement::parse(at).expect("a requirement"),
    }
}

#[test]
fn of_a_crate_used_at_two_versions_judges_it_from_the_oldest() {
    // Arrange
    let workspace = Workspace {
        members: vec![
            member("app", vec![on("syn", "2.0.119")]),
            member("xtask", vec![on("syn", "1.0.109")]),
        ],
        lock: vec![locked("syn", "1.0.109"), locked("syn", "2.0.119")],
    };

    // Act
    let direct = DirectDependencies.of(&workspace);

    // Assert
    assert_eq!(direct.len(), 1);
    assert_eq!(direct[0].version, version("1.0.109"));
}

#[test]
fn of_leaves_out_a_dependency_the_lock_holds_no_registry_package_for() {
    // Arrange
    let workspace = Workspace {
        members: vec![member("app", vec![on("app-core", "0.1.0")])],
        lock: Vec::new(),
    };

    // Act
    let direct = DirectDependencies.of(&workspace);

    // Assert
    assert_eq!(direct, Vec::new());
}

#[test]
fn of_lists_each_registry_dependency_once_with_the_members_using_it() {
    // Arrange
    let workspace = Workspace {
        members: vec![
            member("app", vec![on("serde", "1.0.229"), on("clap", "4.6.6")]),
            member("app-core", vec![on("serde", "1.0.229")]),
        ],
        lock: vec![locked("clap", "4.6.6"), locked("serde", "1.0.229")],
    };

    // Act
    let direct = DirectDependencies.of(&workspace);

    // Assert
    assert_eq!(
        direct,
        vec![
            DirectDependency {
                name: String::from("clap"),
                version: version("4.6.6"),
                members: vec![String::from("app")],
            },
            DirectDependency {
                name: String::from("serde"),
                version: version("1.0.229"),
                members: vec![String::from("app"), String::from("app-core")],
            },
        ]
    );
}

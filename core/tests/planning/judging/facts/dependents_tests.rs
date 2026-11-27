// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::surveyed::surveyed;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::planning::judging::dependent::Dependent;
use ripe4rust_core::planning::judging::facts::dependents::Dependents;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::values::requirement::Requirement;

fn dependent(name: &str, at: &str, requires: (&str, &str, &str)) -> Dependent {
    let (crate_name, requirement, in_use) = requires;
    Dependent {
        name: String::from(name),
        version: version(at),
        requires: String::from(crate_name),
        requirement: Requirement::parse(requirement).expect("a requirement"),
        in_use: version(in_use),
        target: version("0.14.6"),
    }
}

fn requiring(name: &str, at: &str, requires: (&str, &str, &str)) -> LockedPackage {
    let (crate_name, requirement, in_use) = requires;
    LockedPackage {
        name: String::from(name),
        version: version(at),
        dependencies: vec![ResolvedDependency {
            name: String::from(crate_name),
            version: version(in_use),
            requirement: Requirement::parse(requirement).expect("a requirement"),
        }],
    }
}

fn workspace(dependents: Vec<LockedPackage>) -> Workspace {
    let mut workspace = surveyed().workspace;
    workspace.lock.push(LockedPackage {
        name: String::from("prost"),
        version: version("0.13.5"),
        dependencies: Vec::new(),
    });
    workspace.lock.extend(dependents);
    workspace
}

#[test]
fn holding_leaves_out_a_dependent_of_another_copy_of_the_crate() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "legacy",
        "1.0.0",
        ("prost", "^0.12", "0.12.6"),
    )]);

    // Act
    let holding = Dependents::new(&workspace).holding(&breaking("prost", "0.13.5", "0.14.6"));

    // Assert
    assert_eq!(holding, Vec::new());
}

#[test]
fn holding_leaves_out_a_dependent_the_unit_moves_with_it() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "prost-derive",
        "0.13.5",
        ("prost", "^0.13", "0.13.5"),
    )]);
    let mut group = breaking("prost", "0.13.5", "0.14.6");
    group.members.push(String::from("prost-derive"));
    group.moves.push(moved("prost-derive", "0.13.5", "0.14.6"));

    // Act
    let holding = Dependents::new(&workspace).holding(&group);

    // Assert
    assert_eq!(holding, Vec::new());
}

#[test]
fn holding_of_a_breaking_unit_a_dependent_requires_on_its_line_names_the_dependent() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "tonic",
        "0.12.3",
        ("prost", "^0.13", "0.13.5"),
    )]);

    // Act
    let holding = Dependents::new(&workspace).holding(&breaking("prost", "0.13.5", "0.14.6"));

    // Assert
    assert_eq!(
        holding,
        vec![dependent("tonic", "0.12.3", ("prost", "^0.13", "0.13.5"))]
    );
}

#[test]
fn holding_of_a_breaking_unit_names_every_dependent_by_name() {
    // Arrange
    let workspace = workspace(vec![
        requiring("tonic", "0.12.3", ("prost", "^0.13", "0.13.5")),
        requiring("prost-build", "0.13.5", ("prost", "^0.13.5", "0.13.5")),
    ]);

    // Act
    let holding = Dependents::new(&workspace).holding(&breaking("prost", "0.13.5", "0.14.6"));

    // Assert
    assert_eq!(
        holding,
        vec![
            dependent("prost-build", "0.13.5", ("prost", "^0.13.5", "0.13.5")),
            dependent("tonic", "0.12.3", ("prost", "^0.13", "0.13.5")),
        ]
    );
}

#[test]
fn holding_of_a_breaking_unit_whose_dependents_allow_its_target_is_none() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "tonic",
        "0.12.3",
        ("prost", ">=0.13", "0.13.5"),
    )]);

    // Act
    let holding = Dependents::new(&workspace).holding(&breaking("prost", "0.13.5", "0.14.6"));

    // Assert
    assert_eq!(holding, Vec::new());
}

#[test]
fn holding_of_a_compatible_unit_is_none() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "tonic",
        "0.12.3",
        ("prost", "^0.13", "0.13.5"),
    )]);

    // Act
    let holding = Dependents::new(&workspace).holding(&compatible(
        "prost",
        vec![moved("prost", "0.13.5", "0.14.6")],
    ));

    // Assert
    assert_eq!(holding, Vec::new());
}

#[test]
fn holding_of_an_advisory_row_names_the_dependents_of_the_move_held_out_of_it() {
    // Arrange
    let workspace = workspace(vec![requiring(
        "tonic",
        "0.12.3",
        ("prost", "^0.13", "0.13.5"),
    )]);
    let mut row = Concern::Yanked {
        name: String::from("prost"),
        version: version("0.13.5"),
    }
    .row();
    row.held = vec![moved("prost", "0.13.5", "0.14.6")];

    // Act
    let holding = Dependents::new(&workspace).holding(&row);

    // Assert
    assert_eq!(
        holding,
        vec![dependent("tonic", "0.12.3", ("prost", "^0.13", "0.13.5"))]
    );
}

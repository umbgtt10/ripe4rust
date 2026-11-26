// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::requirement_finder::RequirementFinder;
use ripe4rust_core::values::location::Location;

const MANIFEST: &str = "[package]\nname = \"app\"\n\n[dependencies]\nwidget = \"1.4\"\ngadget = { version = \"0.13\", features = [\"net\"] }\nlocal = { path = \"../local\" }\ninherited.workspace = true\nrenamed = { package = \"gizmo\", version = \"2.7\" }\n\n[dev-dependencies.widget]\nversion = \"1.4.1\"\n\n[target.'cfg(unix)'.build-dependencies]\nwidget = \"1\"\n";

fn at(line: u32) -> Location {
    Location {
        file: String::from("app/Cargo.toml"),
        line,
    }
}

fn lines(name: &str) -> Vec<Location> {
    RequirementFinder
        .lines("app/Cargo.toml", MANIFEST, name)
        .expect("lines")
}

#[test]
fn lines_of_a_crate_renamed_are_found_by_its_package() {
    // Arrange & Act
    let found = (lines("gizmo"), lines("renamed"));

    // Assert
    assert_eq!(found, (vec![at(9)], Vec::new()));
}

#[test]
fn lines_of_a_crate_with_no_version_of_its_own_are_none() {
    // Arrange & Act
    let found = (lines("local"), lines("inherited"));

    // Assert
    assert_eq!(found, (Vec::new(), Vec::new()));
}

#[test]
fn lines_of_a_crate_written_in_every_table_are_each_version_s_line() {
    // Arrange & Act
    let found = lines("widget");

    // Assert
    assert_eq!(found, vec![at(5), at(12), at(15)]);
}

#[test]
fn lines_of_a_crate_written_inline_are_its_version_s_line() {
    // Arrange & Act
    let found = lines("gadget");

    // Assert
    assert_eq!(found, vec![at(6)]);
}

#[test]
fn lines_of_the_workspace_s_own_dependencies_are_found_too() {
    // Arrange & Act
    let found = RequirementFinder.lines(
        "Cargo.toml",
        "[workspace]\n\n[workspace.dependencies]\nwidget = \"1.4\"\n",
        "widget",
    );

    // Assert
    assert_eq!(
        found,
        Ok(vec![Location {
            file: String::from("Cargo.toml"),
            line: 4,
        }])
    );
}

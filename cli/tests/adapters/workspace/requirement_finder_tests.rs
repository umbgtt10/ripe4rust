// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::requirement_finder::RequirementFinder;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::written_line::WrittenLine;
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

fn written(line: u32, key: &str, value: &str) -> WrittenLine {
    WrittenLine {
        at: at(line),
        key: String::from(key),
        value: String::from(value),
    }
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

#[test]
fn written_of_a_manifest_is_every_requirement_with_its_crate_and_value() {
    // Arrange & Act
    let found = RequirementFinder.written("app/Cargo.toml", MANIFEST);

    // Assert
    assert_eq!(
        found,
        Ok(vec![
            written(5, "widget", "1.4"),
            written(6, "gadget", "0.13"),
            written(9, "gizmo", "2.7"),
            written(12, "widget", "1.4.1"),
            written(15, "widget", "1"),
        ])
    );
}

#[test]
fn written_of_a_manifest_that_does_not_parse_is_unreadable() {
    // Arrange & Act
    let found = RequirementFinder.written("app/Cargo.toml", "[dependencies\n");

    // Assert
    assert!(
        matches!(found, Err(PortFailure::WorkspaceUnreadable { ref file, .. }) if file == "app/Cargo.toml"),
        "{found:?}"
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::rust_version_finder::RustVersionFinder;
use ripe4rust_core::values::location::Location;

const ROOT: &str = "[workspace]\n\n[workspace.package]\nrust-version = \"1.85\"\n";

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

#[test]
fn line_of_a_member_declaring_its_own_is_its_line() {
    // Arrange & Act
    let line = RustVersionFinder.line(
        "app/Cargo.toml",
        "[package]\nname = \"app\"\nrust-version = \"1.88\"\n",
        ROOT,
    );

    // Assert
    assert_eq!(line, Ok(Some(at("app/Cargo.toml", 3))));
}

#[test]
fn line_of_a_member_declaring_none_is_none() {
    // Arrange & Act
    let line = RustVersionFinder.line("app/Cargo.toml", "[package]\nname = \"app\"\n", ROOT);

    // Assert
    assert_eq!(line, Ok(None));
}

#[test]
fn line_of_a_member_inheriting_it_is_the_workspace_s() {
    // Arrange & Act
    let line = [
        "[package]\nrust-version.workspace = true\n",
        "[package]\nrust-version = { workspace = true }\n",
    ]
    .map(|member| RustVersionFinder.line("app/Cargo.toml", member, ROOT));

    // Assert
    assert_eq!(
        line,
        [Ok(Some(at("Cargo.toml", 4))), Ok(Some(at("Cargo.toml", 4)))]
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::port_failure::PortFailure;

#[test]
fn line_of_an_offset_counts_the_lines_before_it() {
    // Arrange
    let text = "[a]\nb = 1\n\nc = 2\n";

    // Act
    let lines = [0, 4, 11].map(|offset| TomlFile::new("Cargo.toml", text).line(offset));

    // Assert
    assert_eq!(lines, [1, 2, 4]);
}

#[test]
fn table_of_a_file_that_does_not_parse_is_unreadable_where_it_breaks() {
    // Arrange & Act
    let table = TomlFile::new("app/Cargo.toml", "[package]\nname = \n").table();

    // Assert
    assert!(
        matches!(&table, Err(PortFailure::WorkspaceUnreadable { file, detail }) if file == "app/Cargo.toml" && detail.starts_with("line 2: ")),
        "{table:?}"
    );
}

#[test]
fn table_of_a_file_that_parses_is_its_table() {
    // Arrange & Act
    let table = TomlFile::new("Cargo.toml", "[workspace]\nmembers = []\n").table();

    // Assert
    assert_eq!(table.map(|table| table.get_ref().len()), Ok(1));
}

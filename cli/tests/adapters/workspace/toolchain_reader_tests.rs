// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::toolchain_reader::ToolchainReader;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::values::location::Location;

fn at(line: u32) -> Location {
    Location {
        file: String::from("rust-toolchain.toml"),
        line,
    }
}

#[test]
fn file_of_a_toolchain_naming_no_channel_has_an_empty_one_at_its_table() {
    // Arrange & Act
    let file = ToolchainReader.file("# components only\n[toolchain]\ncomponents = [\"clippy\"]\n");

    // Assert
    assert_eq!(
        file,
        Ok(ToolchainFile {
            channel: String::new(),
            at: at(2),
        })
    );
}

#[test]
fn file_of_the_recorded_toolchain_is_its_channel_and_line() {
    // Arrange & Act
    let file = ToolchainReader.file(include_str!(
        "../../fixtures/workspace/files/rust-toolchain.toml.txt"
    ));

    // Assert
    assert_eq!(
        file,
        Ok(ToolchainFile {
            channel: String::from("1.99.1"),
            at: at(2),
        })
    );
}

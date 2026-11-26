// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::location::Location;

#[test]
fn fmt_writes_file_colon_line() {
    // Arrange & Act
    let text = Location {
        file: String::from("Cargo.toml"),
        line: 41,
    }
    .to_string();

    // Assert
    assert_eq!(text, "Cargo.toml:41");
}

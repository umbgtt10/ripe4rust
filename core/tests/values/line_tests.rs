// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::line::Line;

#[test]
fn fmt_of_a_major_line_writes_its_major() {
    // Arrange & Act
    let text = Line {
        major: 2,
        minor: None,
    }
    .to_string();

    // Assert
    assert_eq!(text, "2.x");
}

#[test]
fn fmt_of_a_minor_line_writes_its_minor() {
    // Arrange & Act
    let text = Line {
        major: 0,
        minor: Some(13),
    }
    .to_string();

    // Assert
    assert_eq!(text, "0.13.x");
}

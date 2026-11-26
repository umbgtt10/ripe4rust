// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::round::holding::toml_text::TomlText;

#[test]
fn basic_of_control_characters_escapes_each() {
    // Arrange & Act
    let quoted = TomlText.basic("a\tb\nc\rd\u{8}e\u{c}f\u{1}g\u{7f}");

    // Assert
    assert_eq!(quoted, r#""a\tb\nc\rd\be\ff\u0001g\u007F""#);
}

#[test]
fn basic_of_plain_words_quotes_them_as_they_are() {
    // Arrange & Act
    let quoted = TomlText.basic("stage 2 failed: crap4rust 17 > 15, 'é' kept");

    // Assert
    assert_eq!(quoted, "\"stage 2 failed: crap4rust 17 > 15, 'é' kept\"");
}

#[test]
fn basic_of_quotes_and_backslashes_escapes_them() {
    // Arrange & Act
    let quoted = TomlText.basic(r#"error: expected "a" in C:\work\ws"#);

    // Assert
    assert_eq!(quoted, r#""error: expected \"a\" in C:\\work\\ws""#);
}

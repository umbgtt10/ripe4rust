// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::fmt::Write;

/// Text written as a TOML basic string: in quotes, a quote and a backslash
/// escaped, and every control character too -- by its own escape where TOML
/// has one, as `\uXXXX` otherwise.
pub struct TomlText;

impl TomlText {
    pub fn basic(&self, text: &str) -> String {
        let mut quoted = String::from("\"");
        for character in text.chars() {
            match character {
                '"' => quoted.push_str("\\\""),
                '\\' => quoted.push_str("\\\\"),
                '\n' => quoted.push_str("\\n"),
                '\t' => quoted.push_str("\\t"),
                '\r' => quoted.push_str("\\r"),
                '\u{8}' => quoted.push_str("\\b"),
                '\u{c}' => quoted.push_str("\\f"),
                control if control.is_control() => {
                    let _ = write!(quoted, "\\u{:04X}", u32::from(control));
                }
                other => quoted.push(other),
            }
        }
        quoted.push('"');
        quoted
    }
}

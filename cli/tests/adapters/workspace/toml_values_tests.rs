// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::toml_file::TomlFile;
use cargo_ripe4rust::adapters::workspace::toml_values::TomlValues;
use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::values::location::Location;

const TEXT: &str = "count = 30\nhuge = 99999999999999999999\nratio = 0.5\nfrozen = 2027-01-15\non = true\nnames = [\"a\", 1]\n\n[pins]\nsyn = { reason = \"r\" }\n";

fn at(line: u32) -> Location {
    Location {
        file: String::from("Cargo.toml"),
        line,
    }
}

fn entry(key: &str, value: PolicyValue, line: u32) -> PolicyEntry {
    PolicyEntry {
        key: String::from(key),
        value,
        at: at(line),
    }
}

#[test]
fn at_of_an_offset_is_its_file_and_line() {
    // Arrange & Act
    let at_pins = TomlValues::new("Cargo.toml", TEXT).at(TEXT.find("[pins]").expect("the table"));

    // Assert
    assert_eq!(at_pins, at(8));
}

#[test]
fn entries_of_a_table_are_every_key_with_its_value_and_line() {
    // Arrange
    let table = TomlFile::new("Cargo.toml", TEXT).table().expect("a table");

    // Act
    let entries = TomlValues::new("Cargo.toml", TEXT).entries(table.get_ref());

    // Assert
    assert_eq!(
        entries,
        vec![
            entry("count", PolicyValue::Integer(30), 1),
            entry("frozen", PolicyValue::Text(String::from("2027-01-15")), 4),
            entry(
                "huge",
                PolicyValue::Text(String::from("99999999999999999999")),
                2
            ),
            entry(
                "names",
                PolicyValue::List(vec![
                    PolicyValue::Text(String::from("a")),
                    PolicyValue::Integer(1)
                ]),
                6
            ),
            entry("on", PolicyValue::Boolean(true), 5),
            entry(
                "pins",
                PolicyValue::Table(vec![entry(
                    "syn",
                    PolicyValue::Table(vec![entry(
                        "reason",
                        PolicyValue::Text(String::from("r")),
                        9
                    )]),
                    9
                )]),
                8
            ),
            entry("ratio", PolicyValue::Text(String::from("0.5")), 3),
        ]
    );
}

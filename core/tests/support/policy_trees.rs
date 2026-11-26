// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::values::location::Location;

pub fn at(line: u32) -> Location {
    Location {
        file: String::from("Cargo.toml"),
        line,
    }
}

pub fn entry(key: &str, value: PolicyValue, line: u32) -> PolicyEntry {
    PolicyEntry {
        key: String::from(key),
        value,
        at: at(line),
    }
}

pub fn text(value: &str) -> PolicyValue {
    PolicyValue::Text(String::from(value))
}

pub fn texts(values: &[&str]) -> PolicyValue {
    PolicyValue::List(values.iter().map(|value| text(value)).collect())
}

pub fn tree(entries: Vec<PolicyEntry>) -> PolicyTree {
    PolicyTree {
        at: Some(at(30)),
        entries,
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::policy_entry::PolicyEntry;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::values::location::Location;
use toml::Spanned;
use toml::de::DeTable;
use toml::de::DeValue;

/// TOML values as the policy's raw tree holds them, each key at its line: a
/// string, a date or a float as text -- the readers then say what it should
/// have been --, an integer too large as text, a boolean, a list, a table.
pub struct TomlValues<'t> {
    file: String,
    toml: TomlFile<'t>,
}

impl<'t> TomlValues<'t> {
    pub fn new(file: &str, text: &'t str) -> Self {
        Self {
            file: String::from(file),
            toml: TomlFile::new(file, text),
        }
    }

    pub fn at(&self, offset: usize) -> Location {
        Location {
            file: self.file.clone(),
            line: self.toml.line(offset),
        }
    }

    pub fn entries(&self, table: &DeTable<'_>) -> Vec<PolicyEntry> {
        table
            .iter()
            .map(|(key, value)| PolicyEntry {
                key: key.get_ref().to_string(),
                value: self.value(value),
                at: self.at(key.span().start),
            })
            .collect()
    }

    pub fn value(&self, value: &Spanned<DeValue<'_>>) -> PolicyValue {
        match value.get_ref() {
            DeValue::String(text) => PolicyValue::Text(text.to_string()),
            DeValue::Integer(integer) => i64::from_str_radix(integer.as_str(), integer.radix())
                .map_or_else(
                    |_| PolicyValue::Text(integer.to_string()),
                    PolicyValue::Integer,
                ),
            DeValue::Float(float) => PolicyValue::Text(float.to_string()),
            DeValue::Boolean(boolean) => PolicyValue::Boolean(*boolean),
            DeValue::Datetime(datetime) => PolicyValue::Text(datetime.to_string()),
            DeValue::Array(items) => {
                PolicyValue::List(items.iter().map(|item| self.value(item)).collect())
            }
            DeValue::Table(table) => PolicyValue::Table(self.entries(table)),
        }
    }
}

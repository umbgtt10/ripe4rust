// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::policy_entry::PolicyEntry;
use crate::inputs::policy_value::PolicyValue;
use crate::values::date::Date;
use crate::values::requirement::Requirement;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Reads one section's values, each the kind it must be or a diagnostic
/// naming the key, the section and what it should have been.
pub struct PolicyValues {
    within: String,
}

impl PolicyValues {
    pub fn new(within: &str) -> Self {
        Self {
            within: String::from(within),
        }
    }

    pub fn count(&self, entry: &PolicyEntry) -> Result<u32, Diagnostic> {
        match entry.value {
            PolicyValue::Integer(number) => u32::try_from(number).ok(),
            _ => None,
        }
        .ok_or_else(|| self.invalid(entry, "a whole number"))
    }

    pub fn date(&self, entry: &PolicyEntry) -> Result<Date, Diagnostic> {
        self.parsed(entry, "a date, YYYY-MM-DD", |text| Date::parse(text).ok())
    }

    pub fn invalid(&self, entry: &PolicyEntry, expected: &str) -> Diagnostic {
        DiagnosticCode::PolicyInvalidValue.diagnose(
            Some(entry.at.clone()),
            format!("`{}` in {} is not {expected}", entry.key, self.within),
            format!("write {expected} for `{}`", entry.key),
        )
    }

    pub fn requirement(&self, entry: &PolicyEntry) -> Result<Requirement, Diagnostic> {
        self.parsed(entry, "a version requirement, such as `>=0.32`", |text| {
            Requirement::parse(text).ok()
        })
    }

    pub fn table<'a>(&self, entry: &'a PolicyEntry) -> Result<&'a [PolicyEntry], Diagnostic> {
        match &entry.value {
            PolicyValue::Table(entries) => Ok(entries.as_slice()),
            _ => Err(self.invalid(entry, "a table")),
        }
    }

    pub fn text(&self, entry: &PolicyEntry) -> Result<String, Diagnostic> {
        self.parsed(entry, "a text", |text| Some(String::from(text)))
    }

    pub fn texts(&self, entry: &PolicyEntry) -> Result<Vec<String>, Diagnostic> {
        match &entry.value {
            PolicyValue::List(items) => items
                .iter()
                .map(|item| match item {
                    PolicyValue::Text(text) => Some(text.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<String>>>(),
            _ => None,
        }
        .ok_or_else(|| self.invalid(entry, "a list of texts"))
    }

    pub fn unknown(&self, entries: &[PolicyEntry], known: &[&str]) -> Vec<Diagnostic> {
        entries
            .iter()
            .filter(|entry| !known.contains(&entry.key.as_str()))
            .map(|entry| {
                DiagnosticCode::PolicyUnknownKey.diagnose(
                    Some(entry.at.clone()),
                    format!("`{}` is not a key ripe4rust knows in {}", entry.key, self.within),
                    String::from(
                        "correct its spelling, or remove it; a key newer than this ripe4rust needs a newer release of it",
                    ),
                )
            })
            .collect()
    }

    fn parsed<T>(
        &self,
        entry: &PolicyEntry,
        expected: &str,
        parse: impl Fn(&str) -> Option<T>,
    ) -> Result<T, Diagnostic> {
        match &entry.value {
            PolicyValue::Text(text) => parse(text),
            _ => None,
        }
        .ok_or_else(|| self.invalid(entry, expected))
    }
}

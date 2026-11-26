// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_entry::PolicyEntry;
use crate::inputs::policy_value::PolicyValue;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const RELEASE_KEYS: [&str; 2] = ["crate", "version"];

/// A pin's `until`: `{ crate = "...", version = ">=..." }`, or words.
pub struct UntilReader {
    within: String,
}

impl UntilReader {
    pub fn new(within: &str) -> Self {
        Self {
            within: String::from(within),
        }
    }

    pub fn read(&self, entry: &PolicyEntry) -> Reading<Option<ReleaseCondition>> {
        let values = PolicyValues::new(&self.within);
        match &entry.value {
            PolicyValue::Text(words) if !words.trim().is_empty() => Reading {
                value: Some(ReleaseCondition::Words(words.clone())),
                diagnostics: Vec::new(),
            },
            PolicyValue::Table(keys) => self.release(entry, keys),
            _ => Reading {
                value: None,
                diagnostics: vec![
                    values.invalid(entry, "a table of `crate` and `version`, or words"),
                ],
            },
        }
    }

    fn release(
        &self,
        entry: &PolicyEntry,
        keys: &[PolicyEntry],
    ) -> Reading<Option<ReleaseCondition>> {
        let values = PolicyValues::new(&self.within);
        let mut diagnostics = PolicyValues::new(&format!("the `until` of {}", self.within))
            .unknown(keys, &RELEASE_KEYS);
        let name = keys
            .iter()
            .find(|key| key.key == "crate")
            .and_then(|key| values.text(key).ok());
        let requirement = keys
            .iter()
            .find(|key| key.key == "version")
            .and_then(|key| values.requirement(key).ok());
        let condition = name
            .zip(requirement)
            .map(|(name, requirement)| ReleaseCondition::Release { name, requirement });
        if condition.is_none() {
            diagnostics.push(values.invalid(entry, "a table of `crate` and `version`"));
        }
        Reading {
            value: condition.filter(|_| diagnostics.is_empty()),
            diagnostics,
        }
    }
}

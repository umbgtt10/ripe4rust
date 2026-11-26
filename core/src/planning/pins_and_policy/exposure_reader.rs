// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_entry::PolicyEntry;
use crate::inputs::policy_value::PolicyValue;
use crate::planning::pins_and_policy::exposure_override::ExposureOverride;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const SECTION: &str = "`[workspace.metadata.ripe4rust]`";
const WITHIN: &str = "`exposure`";

/// `exposure = { serde = "public", anyhow = "private" }`: where the parse is
/// wrong about a crate.
pub struct ExposureReader;

impl ExposureReader {
    pub fn read(&self, entries: &[PolicyEntry]) -> Reading<Vec<ExposureOverride>> {
        let Some(exposure) = entries.iter().find(|entry| entry.key == "exposure") else {
            return Reading {
                value: Vec::new(),
                diagnostics: Vec::new(),
            };
        };
        let crates = match PolicyValues::new(SECTION).table(exposure) {
            Ok(crates) => crates,
            Err(diagnostic) => {
                return Reading {
                    value: Vec::new(),
                    diagnostics: vec![diagnostic],
                };
            }
        };
        let values = PolicyValues::new(WITHIN);
        let (read, refused): (Vec<_>, Vec<_>) = crates
            .iter()
            .map(|entry| match &entry.value {
                PolicyValue::Text(word) if word == "public" || word == "private" => {
                    Ok(ExposureOverride {
                        name: String::from(entry.key.as_str()),
                        public: word == "public",
                    })
                }
                _ => Err(values.invalid(entry, "`public` or `private`")),
            })
            .partition(Result::is_ok);
        Reading {
            value: read.into_iter().filter_map(Result::ok).collect(),
            diagnostics: refused.into_iter().filter_map(Result::err).collect(),
        }
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::policy_entry::PolicyEntry;
use crate::inputs::policy_tree::PolicyTree;
use crate::planning::pins_and_policy::exposure_reader::ExposureReader;
use crate::planning::pins_and_policy::pins_reader::PinsReader;
use crate::planning::pins_and_policy::policy::Policy;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use crate::planning::pins_and_policy::thresholds_reader::ThresholdsReader;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const SECTION: &str = "`[workspace.metadata.ripe4rust]`";
const KEYS: [&str; 13] = [
    "exposure",
    "freeze-until",
    "gates",
    "hold-days",
    "moderate-footprint",
    "pins",
    "published",
    "small-footprint",
    "stale-months",
    "toolchain-young-days",
    "validate-beyond-gates",
    "young-days",
    "young-releases",
];

/// The whole policy, read before any rule runs: every key known, every value
/// the kind it must be, every pin complete. A key it does not know is an
/// error, not ignored.
pub struct PolicyReader;

impl PolicyReader {
    pub fn read(&self, tree: &PolicyTree) -> Reading<Policy> {
        let values = PolicyValues::new(SECTION);
        let entries = tree.entries.as_slice();
        let gates = Self::list(&values, entries, "gates");
        let published = Self::list(&values, entries, "published");
        let targets = Self::list(&values, entries, "validate-beyond-gates");
        let freeze = Self::freeze(&values, entries);
        let exposure = ExposureReader.read(entries);
        let thresholds = ThresholdsReader.read(entries);
        let pins = PinsReader.read(entries);
        let mut diagnostics = values.unknown(entries, &KEYS);
        for each in [
            gates.diagnostics,
            published.diagnostics,
            targets.diagnostics,
            freeze.diagnostics,
            exposure.diagnostics,
            thresholds.diagnostics,
            pins.diagnostics,
        ] {
            diagnostics.extend(each);
        }
        if gates.value.is_empty() {
            let at = entries
                .iter()
                .find(|entry| entry.key == "gates")
                .map_or_else(|| tree.at.clone(), |entry| Some(entry.at.clone()));
            diagnostics.push(DiagnosticCode::GatesUndeclared.diagnose(
                at,
                String::from(
                    "no `gates` are declared, so nothing can validate a unit: every unit that would be ripe is asked",
                ),
                String::from(
                    "ask the owner for the commands that validate a change, and write them as `gates = [...]` in `[workspace.metadata.ripe4rust]`",
                ),
            ));
        }
        Reading {
            value: Policy {
                at: tree.at.clone(),
                gates: gates.value,
                published: published.value,
                validate_beyond_gates: targets.value,
                freeze_until: freeze.value,
                exposure: exposure.value,
                thresholds: thresholds.value,
                pins: pins.value,
            },
            diagnostics,
        }
    }

    fn freeze(values: &PolicyValues, entries: &[PolicyEntry]) -> Reading<Option<Date>> {
        match entries
            .iter()
            .find(|entry| entry.key == "freeze-until")
            .map(|entry| values.date(entry))
        {
            Some(Ok(date)) => Reading {
                value: Some(date),
                diagnostics: Vec::new(),
            },
            Some(Err(diagnostic)) => Reading {
                value: None,
                diagnostics: vec![diagnostic],
            },
            None => Reading {
                value: None,
                diagnostics: Vec::new(),
            },
        }
    }

    fn list(values: &PolicyValues, entries: &[PolicyEntry], key: &str) -> Reading<Vec<String>> {
        match entries
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| values.texts(entry))
        {
            Some(Ok(list)) => Reading {
                value: list,
                diagnostics: Vec::new(),
            },
            Some(Err(diagnostic)) => Reading {
                value: Vec::new(),
                diagnostics: vec![diagnostic],
            },
            None => Reading {
                value: Vec::new(),
                diagnostics: Vec::new(),
            },
        }
    }
}

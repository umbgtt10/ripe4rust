// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::inputs::policy_entry::PolicyEntry;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use crate::planning::pins_and_policy::thresholds::Thresholds;
use alloc::vec::Vec;

const SECTION: &str = "`[workspace.metadata.ripe4rust]`";
const KEYS: [&str; 7] = [
    "young-days",
    "young-releases",
    "toolchain-young-days",
    "stale-months",
    "small-footprint",
    "moderate-footprint",
    "hold-days",
];
const DEFAULTS: Thresholds = Thresholds {
    young_days: 30,
    young_releases: 3,
    toolchain_young_days: 0,
    stale_months: 6,
    small_footprint: 10,
    moderate_footprint: 100,
    hold_days: 14,
};

/// The thresholds, each the policy's whole number or its default.
pub struct ThresholdsReader;

impl ThresholdsReader {
    pub fn read(&self, entries: &[PolicyEntry]) -> Reading<Thresholds> {
        let values = PolicyValues::new(SECTION);
        let mut thresholds = DEFAULTS;
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        for entry in entries
            .iter()
            .filter(|entry| KEYS.contains(&entry.key.as_str()))
        {
            match values.count(entry) {
                Ok(count) => thresholds = Self::with(thresholds, &entry.key, count),
                Err(diagnostic) => diagnostics.push(diagnostic),
            }
        }
        Reading {
            value: thresholds,
            diagnostics,
        }
    }

    fn with(thresholds: Thresholds, key: &str, count: u32) -> Thresholds {
        match key {
            "young-days" => Thresholds {
                young_days: count,
                ..thresholds
            },
            "young-releases" => Thresholds {
                young_releases: count,
                ..thresholds
            },
            "toolchain-young-days" => Thresholds {
                toolchain_young_days: count,
                ..thresholds
            },
            "stale-months" => Thresholds {
                stale_months: count,
                ..thresholds
            },
            "small-footprint" => Thresholds {
                small_footprint: count,
                ..thresholds
            },
            "moderate-footprint" => Thresholds {
                moderate_footprint: count,
                ..thresholds
            },
            _ => Thresholds {
                hold_days: count,
                ..thresholds
            },
        }
    }
}

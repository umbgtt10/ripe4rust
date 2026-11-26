// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec;

/// Rule 10: a compatible or toolchain unit nothing above held -- ripe; a
/// red gate turns it into a hold.
pub struct CompatibleOrToolchain;

impl Rule for CompatibleOrToolchain {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let reasons = match facts.kind {
            UnitKind::Toolchain if facts.held_back > 0 => vec![
                String::from("toolchain"),
                format!(
                    "releases {} held back",
                    Wording.count(facts.held_back, "version")
                ),
            ],
            UnitKind::Toolchain => vec![String::from("toolchain")],
            UnitKind::Compatible => facts.rest.map_or_else(
                || vec![String::from("compatible")],
                |moves| vec![format!("{} no unit reaches", Wording.count(moves, "move"))],
            ),
            _ => return None,
        };
        Some(Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(10),
            suggestion: None,
            reasons,
            why: None,
        })
    }
}

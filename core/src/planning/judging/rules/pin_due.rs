// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::traits::rule::Rule;
use alloc::format;
use alloc::vec;

/// Rule 5: a pin holding what the unit moves is released, its condition met
/// or its review date reached -- ask; revisit the pin.
pub struct PinDue;

impl Rule for PinDue {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let (standing, released) = facts.pins.iter().find_map(|standing| {
            standing
                .released
                .as_ref()
                .map(|released| (standing, released))
        })?;
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(5),
            suggestion: Some(Suggestion::RevisitThePin),
            reasons: vec![format!("pinned: {}", standing.pin.reason), released.clone()],
            why: Some(format!(
                "the pin on {} is due: {released}",
                standing.pin.name
            )),
        })
    }
}

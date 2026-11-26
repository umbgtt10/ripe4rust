// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::traits::rule::Rule;
use alloc::format;
use alloc::vec::Vec;

/// Rule 4: pins hold everything the unit would move, and none is released
/// -- ripening, with each pin's reason and what will release it.
pub struct PinHolds;

impl Rule for PinHolds {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let holding = facts.fully_held
            && !facts.pins.is_empty()
            && facts
                .pins
                .iter()
                .all(|standing| standing.released.is_none());
        holding.then(|| Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(4),
            suggestion: None,
            reasons: facts
                .pins
                .iter()
                .flat_map(|standing| {
                    let pin = &standing.pin;
                    let mut reasons = Vec::from([format!("pinned: {}", pin.reason)]);
                    reasons.extend(pin.review.map(|review| format!("review {review}")));
                    reasons.extend(pin.until.as_ref().map(|until| match until {
                        ReleaseCondition::Release { name, requirement } => {
                            format!("until {name} {requirement}")
                        }
                        ReleaseCondition::Words(words) => format!("until {words}"),
                    }));
                    reasons
                })
                .collect(),
            why: None,
        })
    }
}

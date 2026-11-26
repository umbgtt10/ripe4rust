// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use alloc::string::String;

/// With no gates declared nothing can validate a unit, so every unit the
/// rules call ripe is asked, "declare the gates", urgency kept.
pub struct NoGates;

impl NoGates {
    pub fn applied(&self, decision: Decision, facts: &UnitFacts) -> Decision {
        if facts.gates > 0 || decision.outcome != Outcome::Ripe {
            return decision;
        }
        let mut reasons = decision.reasons;
        reasons.push(String::from("no gates declared"));
        Decision {
            outcome: Outcome::Ask,
            urgent: decision.urgent,
            rule: decision.rule,
            suggestion: Some(Suggestion::DeclareTheGates),
            reasons,
            why: Some(String::from(
                "no gates are declared, so nothing can validate it",
            )),
        }
    }
}

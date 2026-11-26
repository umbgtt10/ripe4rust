// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use alloc::format;
use alloc::vec;

/// Rule 1: a unit that clears an advisory or a yanked release and reaches a
/// target the gates cannot validate -- ask, urgent; apply, then validate
/// there, even past a pin or a freeze.
pub struct UrgentBeyondGates;

impl Rule for UrgentBeyondGates {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        if facts.concerns.is_empty() || facts.reaches.is_empty() {
            return None;
        }
        let concerns = Wording.list(&facts.concerns);
        let targets = Wording.list(&facts.reaches);
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(1),
            suggestion: Some(Suggestion::ApplyThenValidate),
            reasons: vec![format!("clears {concerns}"), format!("reaches {targets}")],
            why: Some(format!(
                "it clears {concerns} and reaches {targets}, which the gates cannot validate"
            )),
        })
    }
}

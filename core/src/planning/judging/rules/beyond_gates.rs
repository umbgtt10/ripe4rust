// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::verdict::Verdict;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;

/// Rule 7: the unit reaches a target the gates cannot validate -- ask;
/// apply, then validate there. A breaking unit carries its verdict too.
pub struct BeyondGates;

impl Rule for BeyondGates {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        if facts.reaches.is_empty() {
            return None;
        }
        let targets = Wording.list(&facts.reaches);
        let mut reasons = vec![format!("reaches {targets}")];
        if facts.kind == UnitKind::Breaking {
            reasons.push(Verdict.reason(facts));
        }
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(7),
            suggestion: Some(Suggestion::ApplyThenValidate),
            reasons,
            why: Some(format!(
                "it reaches {targets}, which the gates cannot validate"
            )),
        })
    }
}

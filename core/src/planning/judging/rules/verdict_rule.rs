// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::verdict::Verdict;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;

/// Rule 12: every other breaking unit -- ask; migrate now, plan it or wait,
/// by the verdict.
pub struct VerdictRule;

impl VerdictRule {
    pub fn decision(&self, facts: &UnitFacts) -> Decision {
        let call = Verdict.call(facts);
        let words = call.suggestion.words();
        Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(12),
            suggestion: Some(call.suggestion),
            reasons: vec![format!("{words}: {}", call.reason)],
            why: Some(format!(
                "it is a breaking upgrade: {words}, {}",
                call.reason
            )),
        }
    }
}

impl Rule for VerdictRule {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        (facts.kind == UnitKind::Breaking).then(|| self.decision(facts))
    }
}

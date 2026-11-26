// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::rules::freeze_open::FreezeOpen;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::traits::rule::Rule;
use alloc::format;
use alloc::string::String;

/// Against drift: a unit a pin (rule 4) or a young line (rule 9) would hold
/// is asked instead, overripe, when the workspace is two lines or more
/// behind or the line in use is stale while a newer one is active. A freeze
/// still holds: it ends on its own date.
pub struct Drift;

impl Drift {
    pub fn applied(&self, decision: Decision, facts: &UnitFacts) -> Decision {
        let held = decision.outcome == Outcome::Ripening && matches!(decision.rule, Some(4 | 9));
        match self.escalation(facts) {
            Some(_) if held && facts.freeze.is_some() => {
                FreezeOpen.decide(facts).unwrap_or(decision)
            }
            Some(reason) if held => {
                let mut reasons = decision.reasons;
                reasons.push(format!("overripe: {reason}"));
                Decision {
                    outcome: Outcome::Ask,
                    urgent: false,
                    rule: decision.rule,
                    suggestion: Some(Suggestion::Overripe),
                    reasons,
                    why: Some(format!("it is overripe: {reason}")),
                }
            }
            _ => decision,
        }
    }

    pub fn escalation(&self, facts: &UnitFacts) -> Option<String> {
        if facts.lines_behind >= 2 {
            return Some(format!("{} lines behind", facts.lines_behind));
        }
        facts
            .in_use_line
            .filter(|in_use| !in_use.active && facts.newer_active)
            .map(|in_use| {
                format!(
                    "{} is stale while a newer line is active",
                    in_use.stats.line
                )
            })
    }
}

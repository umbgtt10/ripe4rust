// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::downgrade::Downgrade;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// A move to a lower version inside a unit is never silent: each is named
/// among its reasons, and one leaving a release that is not yanked turns a
/// unit the rules call ripe into an ask, urgency and rule kept -- a move back
/// is a person's call. One leaving a yanked release is the cure: named, the
/// outcome unchanged.
pub struct MovedBack;

impl MovedBack {
    pub fn applied(&self, decision: Decision, facts: &UnitFacts) -> Decision {
        if facts.downgrades.is_empty() {
            return decision;
        }
        let mut reasons = decision.reasons;
        reasons.extend(
            facts
                .downgrades
                .iter()
                .map(|downgrade| format!("moves {}", Self::phrase(downgrade))),
        );
        let asked = facts
            .downgrades
            .iter()
            .filter(|downgrade| !downgrade.yanked)
            .map(Self::phrase)
            .collect::<Vec<String>>();
        if decision.outcome != Outcome::Ripe || asked.is_empty() {
            return Decision {
                reasons,
                ..decision
            };
        }
        Decision {
            outcome: Outcome::Ask,
            suggestion: Some(Suggestion::MovesBack),
            reasons,
            why: Some(format!(
                "it moves {}, and a move to a lower version is a person's call",
                Wording.list(&asked)
            )),
            ..decision
        }
    }

    fn phrase(downgrade: &Downgrade) -> String {
        let left = if downgrade.yanked { "the yanked " } else { "" };
        format!(
            "{} back from {left}{} to {}",
            downgrade.name, downgrade.from, downgrade.to
        )
    }
}

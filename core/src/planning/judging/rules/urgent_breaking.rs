// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::rules::git_unit::GitUnit;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec;

/// Rule 3: a breaking unit that clears an advisory or a yanked release --
/// ask, urgent; migrate now, even past a pin or a freeze. A git unit that
/// clears one is asked urgently too, to take it: a git source is never ripe.
pub struct UrgentBreaking;

impl Rule for UrgentBreaking {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        if facts.concerns.is_empty() {
            return None;
        }
        let concerns = Wording.list(&facts.concerns);
        let (suggestion, kind, why) = match facts.kind {
            UnitKind::Breaking => (
                Some(Suggestion::MigrateNow),
                String::from("breaking"),
                format!("it clears {concerns}, and only a breaking upgrade does"),
            ),
            UnitKind::Git => (
                None,
                GitUnit.reason(),
                format!(
                    "it clears {concerns}, and moves a git source, which makes no version promise"
                ),
            ),
            _ => return None,
        };
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(3),
            suggestion,
            reasons: vec![format!("clears {concerns}"), kind],
            why: Some(why),
        })
    }
}

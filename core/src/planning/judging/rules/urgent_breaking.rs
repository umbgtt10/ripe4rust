// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec;

/// Rule 3: a breaking unit that clears an advisory or a yanked release --
/// ask, urgent; migrate now, even past a pin or a freeze.
pub struct UrgentBreaking;

impl Rule for UrgentBreaking {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        if facts.kind != UnitKind::Breaking || facts.concerns.is_empty() {
            return None;
        }
        let concerns = Wording.list(&facts.concerns);
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(3),
            suggestion: Some(Suggestion::MigrateNow),
            reasons: vec![format!("clears {concerns}"), String::from("breaking")],
            why: Some(format!(
                "it clears {concerns}, and only a breaking upgrade does"
            )),
        })
    }
}

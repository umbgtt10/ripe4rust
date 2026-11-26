// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;

/// Rule 2: a compatible unit that clears an advisory or a yanked release --
/// ripe, urgent, even past a pin or a freeze: the safest upgrade there is,
/// and the most pressing.
pub struct UrgentCompatible;

impl Rule for UrgentCompatible {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        (facts.kind == UnitKind::Compatible && !facts.concerns.is_empty()).then(|| Decision {
            outcome: Outcome::Ripe,
            urgent: true,
            rule: Some(2),
            suggestion: None,
            reasons: vec![format!("clears {}", Wording.list(&facts.concerns))],
            why: None,
        })
    }
}

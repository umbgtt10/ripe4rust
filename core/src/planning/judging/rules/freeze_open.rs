// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::traits::rule::Rule;
use alloc::format;
use alloc::vec;

/// Rule 6: a freeze window is open -- ripening, until it closes.
pub struct FreezeOpen;

impl Rule for FreezeOpen {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        facts.freeze.map(|until| Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(6),
            suggestion: Some(Suggestion::UntilTheWindowCloses),
            reasons: vec![format!("frozen until {until}")],
            why: None,
        })
    }
}

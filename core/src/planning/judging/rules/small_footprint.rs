// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::vec;

/// Rule 11: a breaking unit whose footprint is under `small-footprint` --
/// ripe; a red gate turns it into ask. A footprint not known is never
/// small, nor is one that is a floor, nor a crate named nowhere: possibly an
/// unused dependency, it is asked.
pub struct SmallFootprint;

impl Rule for SmallFootprint {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let footprint = facts.footprint?;
        let small = facts.kind == UnitKind::Breaking
            && footprint.complete
            && footprint.lines > 0
            && footprint.lines < facts.small_footprint;
        small.then(|| Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(11),
            suggestion: None,
            reasons: vec![Wording.footprint(footprint)],
            why: None,
        })
    }
}

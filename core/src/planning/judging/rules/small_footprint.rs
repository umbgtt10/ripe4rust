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

/// Rule 11: a breaking unit whose footprint is under `small-footprint` --
/// ripe; a red gate turns it into ask. A footprint not yet known is never
/// small.
pub struct SmallFootprint;

impl Rule for SmallFootprint {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let footprint = facts.footprint?;
        let small = facts.kind == UnitKind::Breaking && footprint.lines < facts.small_footprint;
        small.then(|| Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(11),
            suggestion: None,
            reasons: vec![format!(
                "footprint {} in {}",
                Wording.count(footprint.lines, "line"),
                Wording.count(footprint.files, "file")
            )],
            why: None,
        })
    }
}

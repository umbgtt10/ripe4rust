// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::unit_kind::UnitKind;

/// A breaking unit whose crate a complete parse finds named nowhere is
/// possibly an unused dependency: its reasons say so, whatever rule decided
/// it, and its question offers to remove the crate -- a person decides, as a
/// crate can be needed for its features or side effects.
pub struct NamedNowhere;

impl NamedNowhere {
    pub fn applied(&self, decision: Decision, facts: &UnitFacts) -> Decision {
        let Some(footprint) = facts.footprint.filter(|_| self.fits(facts)) else {
            return decision;
        };
        let said = Wording.footprint(footprint);
        if decision.reasons.iter().any(|reason| reason.contains(&said)) {
            return decision;
        }
        let mut reasons = decision.reasons;
        reasons.push(said);
        Decision {
            reasons,
            ..decision
        }
    }

    pub fn fits(&self, facts: &UnitFacts) -> bool {
        facts.kind == UnitKind::Breaking
            && facts
                .footprint
                .is_some_and(|footprint| footprint.complete && footprint.lines == 0)
    }
}

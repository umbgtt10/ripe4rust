// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::dependent::Dependent;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// A breaking unit whose crate a dependent's requirement holds on the
/// version in use cannot land alone: moving it only adds the target beside
/// that version. Its reasons name every dependent, whatever decided it; one
/// the rules would take or ask ripens instead, rule kept, until its
/// dependents move -- their own upgrade releases it. An urgent unit keeps
/// its ask, as one is never held, and one already ripening keeps what holds
/// it.
pub struct HeldByDependents;

impl HeldByDependents {
    pub fn applied(&self, decision: Decision, facts: &UnitFacts) -> Decision {
        if facts.held_by.is_empty() {
            return decision;
        }
        let mut reasons = decision.reasons;
        reasons.extend(Self::said(&facts.held_by));
        if decision.urgent || decision.outcome == Outcome::Ripening {
            return Decision {
                reasons,
                ..decision
            };
        }
        Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: decision.rule,
            suggestion: Some(Suggestion::UntilItsDependentsMove),
            reasons,
            why: None,
        }
    }

    fn said(held_by: &[Dependent]) -> Vec<String> {
        let mut crates = held_by
            .iter()
            .map(|dependent| &dependent.requires)
            .collect::<Vec<&String>>();
        crates.dedup();
        crates
            .into_iter()
            .filter_map(|name| {
                let holding = held_by
                    .iter()
                    .filter(|dependent| dependent.requires == *name)
                    .collect::<Vec<&Dependent>>();
                let first = holding.first()?;
                let requiring = holding
                    .iter()
                    .map(|dependent| {
                        format!(
                            "{} {} requires {name} {}",
                            dependent.name, dependent.version, dependent.requirement
                        )
                    })
                    .collect::<Vec<String>>();
                Some(format!(
                    "{}: moving it adds {} beside {}",
                    Wording.list(&requiring),
                    first.target,
                    first.in_use
                ))
            })
            .collect()
    }
}

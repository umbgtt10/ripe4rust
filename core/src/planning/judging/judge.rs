// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::drift::Drift;
use crate::planning::judging::no_gates::NoGates;
use crate::planning::judging::rules::beyond_gates::BeyondGates;
use crate::planning::judging::rules::breaking_change_of_ours::BreakingChangeOfOurs;
use crate::planning::judging::rules::compatible_or_toolchain::CompatibleOrToolchain;
use crate::planning::judging::rules::freeze_open::FreezeOpen;
use crate::planning::judging::rules::pin_due::PinDue;
use crate::planning::judging::rules::pin_holds::PinHolds;
use crate::planning::judging::rules::small_footprint::SmallFootprint;
use crate::planning::judging::rules::urgent_beyond_gates::UrgentBeyondGates;
use crate::planning::judging::rules::urgent_breaking::UrgentBreaking;
use crate::planning::judging::rules::urgent_compatible::UrgentCompatible;
use crate::planning::judging::rules::verdict_rule::VerdictRule;
use crate::planning::judging::rules::young_line::YoungLine;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;

/// The rules, in order, the first that matches deciding; rule 12 takes
/// every breaking unit the others leave. Then against drift, then the
/// no-gates downgrade. An advisory row is asked, urgent, by no rule.
pub struct Judge;

impl Judge {
    pub fn judge(&self, facts: &UnitFacts) -> Decision {
        if facts.kind == UnitKind::Advisory {
            let concerns = Wording.list(&facts.concerns);
            return Decision {
                outcome: Outcome::Ask,
                urgent: true,
                rule: None,
                suggestion: Some(Suggestion::ReplaceOrAccept),
                reasons: vec![format!("{concerns}, which no upgrade clears")],
                why: Some(format!(
                    "{concerns} affects {}, and no upgrade in this plan clears it",
                    facts.name
                )),
            };
        }
        let rules: [&dyn Rule; 11] = [
            &UrgentBeyondGates,
            &UrgentCompatible,
            &UrgentBreaking,
            &PinHolds,
            &PinDue,
            &FreezeOpen,
            &BeyondGates,
            &BreakingChangeOfOurs,
            &YoungLine,
            &CompatibleOrToolchain,
            &SmallFootprint,
        ];
        let decided = rules
            .iter()
            .find_map(|rule| rule.decide(facts))
            .unwrap_or_else(|| VerdictRule.decision(facts));
        NoGates.applied(Drift.applied(decided, facts), facts)
    }
}

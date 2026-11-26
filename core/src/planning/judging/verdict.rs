// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::verdict_call::VerdictCall;
use crate::planning::judging::wording::Wording;
use alloc::format;
use alloc::string::String;

/// The verdict on a breaking unit that asks: migrate now where the line in
/// use is stale while the new one is active, migrating removes a second
/// copy, or the sources name the crate nowhere; plan it where the new line has settled and the footprint is under
/// `moderate-footprint`; wait otherwise, with the reason -- a footprint not
/// known, or a floor, among them.
pub struct Verdict;

impl Verdict {
    pub fn call(&self, facts: &UnitFacts) -> VerdictCall {
        let (Some(in_use), Some(new)) = (facts.in_use_line, facts.new_line) else {
            return Self::waiting(facts);
        };
        if !in_use.active && new.active {
            return VerdictCall {
                suggestion: Suggestion::MigrateNow,
                reason: format!(
                    "{} is stale while {} is active",
                    in_use.stats.line, new.stats.line
                ),
            };
        }
        if facts.removes_copy {
            return VerdictCall {
                suggestion: Suggestion::MigrateNow,
                reason: format!("it removes a second copy of {} from the tree", facts.name),
            };
        }
        if new.young {
            return VerdictCall {
                suggestion: Suggestion::Wait,
                reason: format!("{} is young", new.stats.line),
            };
        }
        Self::waiting(facts)
    }

    pub fn reason(&self, facts: &UnitFacts) -> String {
        let call = self.call(facts);
        format!("{}: {}", call.suggestion.words(), call.reason)
    }

    fn waiting(facts: &UnitFacts) -> VerdictCall {
        let Some(known) = facts.footprint else {
            return VerdictCall {
                suggestion: Suggestion::Wait,
                reason: String::from("footprint not known"),
            };
        };
        let footprint = Wording.footprint(known);
        if !known.complete {
            return VerdictCall {
                suggestion: Suggestion::Wait,
                reason: footprint,
            };
        }
        if known.lines == 0 {
            return VerdictCall {
                suggestion: Suggestion::MigrateNow,
                reason: footprint,
            };
        }
        if known.lines < facts.moderate_footprint {
            VerdictCall {
                suggestion: Suggestion::PlanIt,
                reason: footprint,
            }
        } else {
            VerdictCall {
                suggestion: Suggestion::Wait,
                reason: format!("{footprint}, at or above {}", facts.moderate_footprint),
            }
        }
    }
}

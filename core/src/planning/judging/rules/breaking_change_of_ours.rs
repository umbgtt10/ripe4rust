// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::verdict::Verdict;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Rule 8: a breaking unit whose crate's types are in a published crate's
/// public API, or may be there while that API is not known, or that
/// raises a published crate's `rust-version` -- ask; the upgrade is a
/// breaking change of ours too.
pub struct BreakingChangeOfOurs;

impl BreakingChangeOfOurs {
    fn why(whys: &[String], known: bool) -> String {
        let ending = if known {
            ": the upgrade is a breaking change of ours too"
        } else {
            ", and if they are, the upgrade is a breaking change of ours too"
        };
        format!("{}{ending}", whys.join("; "))
    }
}

impl Rule for BreakingChangeOfOurs {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        if facts.kind != UnitKind::Breaking {
            return None;
        }
        let exposed = facts.exposed_in.iter().map(|member| {
            (
                format!("in {member}'s public API"),
                format!("{}'s types are in {member}'s public API", facts.name),
            )
        });
        let unknown = facts.exposure_unknown_in.iter().map(|member| {
            (
                format!("exposure in {member}'s public API not known"),
                format!(
                    "whether {}'s types are in {member}'s public API is not known",
                    facts.name
                ),
            )
        });
        let raised = facts
            .raises
            .iter()
            .filter(|raise| raise.published)
            .map(|raise| {
                let raising = format!("raises {}'s rust-version to {}", raise.member, raise.to);
                (raising.clone(), format!("it {raising}"))
            });
        let (mut reasons, whys): (Vec<String>, Vec<String>) =
            exposed.chain(unknown).chain(raised).unzip();
        if reasons.is_empty() {
            return None;
        }
        let known = whys.len() > facts.exposure_unknown_in.len();
        reasons.push(Verdict.reason(facts));
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(8),
            suggestion: Some(Suggestion::BreakingChangeOfOurs),
            reasons,
            why: Some(Self::why(&whys, known)),
        })
    }
}

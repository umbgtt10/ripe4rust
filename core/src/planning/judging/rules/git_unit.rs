// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::string::String;
use alloc::vec;

const REASON: &str = "git source: no version promise, a person decides";

/// Rule 13: a git unit no rule above decides -- ask: a git source makes no
/// version promise, so whether to take a new rev is a person's call. Never
/// ripe; urgent only where it clears an advisory or a yanked release, which
/// rule 3 asks.
pub struct GitUnit;

impl GitUnit {
    pub fn reason(&self) -> String {
        String::from(REASON)
    }
}

impl Rule for GitUnit {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        (facts.kind == UnitKind::Git).then(|| Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(13),
            suggestion: None,
            reasons: vec![self.reason()],
            why: Some(String::from(
                "it moves a git source, which makes no version promise: a person decides whether to take it",
            )),
        })
    }
}

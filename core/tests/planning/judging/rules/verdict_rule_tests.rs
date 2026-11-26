// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::verdict_rule::VerdictRule;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_unit_asks_it_with_the_verdict() {
    // Arrange & Act
    let decision = VerdictRule.decide(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(12),
            suggestion: Some(Suggestion::PlanIt),
            reasons: vec![String::from("plan it: footprint 42 lines in 7 files")],
            why: Some(String::from(
                "it is a breaking upgrade: plan it, footprint 42 lines in 7 files"
            )),
        })
    );
}

#[test]
fn decide_of_a_compatible_unit_is_none() {
    // Arrange & Act
    let decision = VerdictRule.decide(&facts(UnitKind::Compatible));

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decision_of_any_unit_is_the_verdict_s() {
    // Arrange & Act
    let decision = VerdictRule.decision(&facts(UnitKind::Toolchain));

    // Assert
    assert_eq!(decision.rule, Some(12));
}

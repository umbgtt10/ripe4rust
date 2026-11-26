// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::urgent_breaking::UrgentBreaking;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_fix_asks_to_migrate_now() {
    // Arrange
    let fix = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = UrgentBreaking.decide(&fix);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(3),
            suggestion: Some(Suggestion::MigrateNow),
            reasons: vec![
                String::from("clears RUSTSEC-2026-0190"),
                String::from("breaking")
            ],
            why: Some(String::from(
                "it clears RUSTSEC-2026-0190, and only a breaking upgrade does"
            )),
        })
    );
}

#[test]
fn decide_of_a_breaking_unit_clearing_nothing_is_none() {
    // Arrange & Act
    let decision = UrgentBreaking.decide(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_git_unit_clearing_a_concern_asks_it_urgently_to_take_it() {
    // Arrange
    let clearing = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0097")],
        ..facts(UnitKind::Git)
    };

    // Act
    let decision = UrgentBreaking.decide(&clearing);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(3),
            suggestion: None,
            reasons: vec![
                String::from("clears RUSTSEC-2026-0097"),
                String::from("git source: no version promise, a person decides")
            ],
            why: Some(String::from(
                "it clears RUSTSEC-2026-0097, and moves a git source, which makes no version promise"
            )),
        })
    );
}

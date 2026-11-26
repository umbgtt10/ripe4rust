// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::urgent_compatible::UrgentCompatible;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_fix_is_none() {
    // Arrange
    let fix = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = UrgentCompatible.decide(&fix);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_compatible_fix_is_ripe_and_urgent() {
    // Arrange
    let fix = UnitFacts {
        concerns: vec![String::from("the yanked spin 0.10.0")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = UrgentCompatible.decide(&fix);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripe,
            urgent: true,
            rule: Some(2),
            suggestion: None,
            reasons: vec![String::from("clears the yanked spin 0.10.0")],
            why: None,
        })
    );
}

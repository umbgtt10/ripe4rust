// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::urgent_beyond_gates::UrgentBeyondGates;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_fix_reaching_a_target_asks_it_urgently() {
    // Arrange
    let fix = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = UrgentBeyondGates.decide(&fix);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: Some(1),
            suggestion: Some(Suggestion::ApplyThenValidate),
            reasons: vec![
                String::from("clears RUSTSEC-2026-0190"),
                String::from("reaches thumbv7em-none-eabihf"),
            ],
            why: Some(String::from(
                "it clears RUSTSEC-2026-0190 and reaches thumbv7em-none-eabihf, which the gates cannot validate"
            )),
        })
    );
}

#[test]
fn decide_of_a_fix_reaching_no_target_is_none() {
    // Arrange
    let fix = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = UrgentBeyondGates.decide(&fix);

    // Assert
    assert_eq!(decision, None);
}

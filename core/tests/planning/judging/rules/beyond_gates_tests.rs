// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::beyond_gates::BeyondGates;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_unit_reaching_a_target_adds_the_verdict() {
    // Arrange
    let reaching = UnitFacts {
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BeyondGates.decide(&reaching);

    // Assert
    assert_eq!(
        decision.map(|decision| decision.reasons),
        Some(vec![
            String::from("reaches thumbv7em-none-eabihf"),
            String::from("plan it: footprint 42 lines in 7 files"),
        ])
    );
}

#[test]
fn decide_of_a_unit_reaching_a_target_asks_it_validated_there() {
    // Arrange
    let reaching = UnitFacts {
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Toolchain)
    };

    // Act
    let decision = BeyondGates.decide(&reaching);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(7),
            suggestion: Some(Suggestion::ApplyThenValidate),
            reasons: vec![String::from("reaches thumbv7em-none-eabihf")],
            why: Some(String::from(
                "it reaches thumbv7em-none-eabihf, which the gates cannot validate"
            )),
        })
    );
}

#[test]
fn decide_of_a_unit_reaching_no_target_is_none() {
    // Arrange & Act
    let decision = BeyondGates.decide(&facts(UnitKind::Compatible));

    // Assert
    assert_eq!(decision, None);
}

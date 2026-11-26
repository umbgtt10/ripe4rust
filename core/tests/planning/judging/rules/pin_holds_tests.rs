// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::pin_holds::PinHolds;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::requirement::Requirement;

#[test]
fn decide_names_a_release_condition_it_waits_for() {
    // Arrange
    let mut pin = pinned(None);
    pin.pin.review = None;
    pin.pin.until = Some(ReleaseCondition::Release {
        name: String::from("widget-tools"),
        requirement: Requirement::parse(">=0.32").expect("a requirement"),
    });
    let held = UnitFacts {
        pins: vec![pin],
        fully_held: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = PinHolds.decide(&held);

    // Assert
    assert_eq!(
        decision.map(|decision| decision.reasons),
        Some(vec![
            String::from("pinned: a migration of every visitor"),
            String::from("until widget-tools >=0.32"),
        ])
    );
}

#[test]
fn decide_of_a_pin_released_is_none() {
    // Arrange
    let due = UnitFacts {
        pins: vec![pinned(Some("review 2026-11-01 reached"))],
        fully_held: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = PinHolds.decide(&due);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_unit_its_pins_hold_is_ripening_with_their_reasons() {
    // Arrange
    let held = UnitFacts {
        pins: vec![pinned(None)],
        fully_held: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = PinHolds.decide(&held);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(4),
            suggestion: None,
            reasons: vec![
                String::from("pinned: a migration of every visitor"),
                String::from("review 2027-01-31"),
            ],
            why: None,
        })
    );
}

#[test]
fn decide_of_a_unit_its_pins_hold_only_in_part_is_none() {
    // Arrange
    let partly = UnitFacts {
        pins: vec![pinned(None)],
        fully_held: false,
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = PinHolds.decide(&partly);

    // Assert
    assert_eq!(decision, None);
}

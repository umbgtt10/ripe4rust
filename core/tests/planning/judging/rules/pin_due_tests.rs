// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::pin_due::PinDue;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_pin_released_asks_to_revisit_it() {
    // Arrange
    let due = UnitFacts {
        pins: vec![pinned(Some("review 2026-11-01 reached"))],
        fully_held: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = PinDue.decide(&due);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(5),
            suggestion: Some(Suggestion::RevisitThePin),
            reasons: vec![
                String::from("pinned: a migration of every visitor"),
                String::from("review 2026-11-01 reached"),
            ],
            why: Some(String::from(
                "the pin on widget is due: review 2026-11-01 reached"
            )),
        })
    );
}

#[test]
fn decide_of_pins_none_released_is_none() {
    // Arrange
    let held = UnitFacts {
        pins: vec![pinned(None)],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = PinDue.decide(&held);

    // Assert
    assert_eq!(decision, None);
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::no_gates::NoGates;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

fn ripe(urgent: bool) -> Decision {
    Decision {
        outcome: Outcome::Ripe,
        urgent,
        rule: Some(if urgent { 2 } else { 10 }),
        suggestion: None,
        reasons: vec![String::from("compatible")],
        why: None,
    }
}

fn ungated() -> UnitFacts {
    UnitFacts {
        gates: 0,
        ..facts(UnitKind::Compatible)
    }
}

#[test]
fn applied_to_a_ripe_unit_without_gates_asks_it() {
    // Arrange & Act
    let decision = NoGates.applied(ripe(false), &ungated());

    // Assert
    assert_eq!(
        decision,
        Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(10),
            suggestion: Some(Suggestion::DeclareTheGates),
            reasons: vec![
                String::from("compatible"),
                String::from("no gates declared")
            ],
            why: Some(String::from(
                "no gates are declared, so nothing can validate it"
            )),
        }
    );
}

#[test]
fn applied_to_an_urgent_ripe_unit_without_gates_keeps_it_urgent() {
    // Arrange & Act
    let decision = NoGates.applied(ripe(true), &ungated());

    // Assert
    assert_eq!(decision.outcome, Outcome::Ask);
    assert!(decision.urgent);
    assert_eq!(decision.rule, Some(2));
}

#[test]
fn applied_with_gates_declared_changes_nothing() {
    // Arrange & Act
    let decision = NoGates.applied(ripe(false), &facts(UnitKind::Compatible));

    // Assert
    assert_eq!(decision, ripe(false));
}

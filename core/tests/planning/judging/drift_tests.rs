// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::standing;
use crate::support::releases::date;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::drift::Drift;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

fn held(rule: u8) -> Decision {
    Decision {
        outcome: Outcome::Ripening,
        urgent: false,
        rule: Some(rule),
        suggestion: Some(Suggestion::UntilItSettles),
        reasons: vec![String::from("3.x is young")],
        why: None,
    }
}

#[test]
fn applied_to_a_hold_not_drifting_keeps_it() {
    // Arrange & Act
    let decision = Drift.applied(held(9), &facts(UnitKind::Breaking));

    // Assert
    assert_eq!(decision, held(9));
}

#[test]
fn applied_to_a_pin_hold_in_a_freeze_keeps_the_freeze() {
    // Arrange
    let frozen = UnitFacts {
        lines_behind: 3,
        freeze: Some(date("2027-01-15")),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = Drift.applied(held(4), &frozen);

    // Assert
    assert_eq!(decision.outcome, Outcome::Ripening);
    assert_eq!(decision.rule, Some(6));
}

#[test]
fn applied_to_a_ripe_unit_changes_nothing() {
    // Arrange
    let ripe = Decision {
        outcome: Outcome::Ripe,
        urgent: false,
        rule: Some(10),
        suggestion: None,
        reasons: vec![String::from("compatible")],
        why: None,
    };
    let behind = UnitFacts {
        lines_behind: 4,
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = Drift.applied(ripe.clone(), &behind);

    // Assert
    assert_eq!(decision, ripe);
}

#[test]
fn applied_to_a_young_hold_two_lines_behind_asks_it_overripe() {
    // Arrange
    let behind = UnitFacts {
        lines_behind: 2,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = Drift.applied(held(9), &behind);

    // Assert
    assert_eq!(
        decision,
        Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(9),
            suggestion: Some(Suggestion::Overripe),
            reasons: vec![
                String::from("3.x is young"),
                String::from("overripe: 2 lines behind")
            ],
            why: Some(String::from("it is overripe: 2 lines behind")),
        }
    );
}

#[test]
fn escalation_of_a_stale_line_with_a_newer_one_active_says_so() {
    // Arrange
    let stale = UnitFacts {
        in_use_line: Some(standing(1, false, false)),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let escalation = Drift.escalation(&stale);

    // Assert
    assert_eq!(
        escalation,
        Some(String::from("1.x is stale while a newer line is active"))
    );
}

#[test]
fn escalation_of_a_stale_line_with_no_newer_one_active_is_none() {
    // Arrange
    let stale = UnitFacts {
        in_use_line: Some(standing(1, false, false)),
        newer_active: false,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let escalation = Drift.escalation(&stale);

    // Assert
    assert_eq!(escalation, None);
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::releases::version;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::downgrade::Downgrade;
use ripe4rust_core::planning::judging::moved_back::MovedBack;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

fn back(name: &str, from: &str, to: &str, yanked: bool) -> Downgrade {
    Downgrade {
        name: String::from(name),
        from: version(from),
        to: version(to),
        yanked,
    }
}

fn decided(outcome: Outcome, urgent: bool) -> Decision {
    Decision {
        outcome,
        urgent,
        rule: Some(10),
        suggestion: None,
        reasons: vec![String::from("compatible")],
        why: None,
    }
}

fn moving_back(downgrades: Vec<Downgrade>) -> UnitFacts {
    UnitFacts {
        downgrades,
        ..facts(UnitKind::Compatible)
    }
}

#[test]
fn applied_to_a_ripe_unit_moving_a_crate_back_asks_it_naming_the_move() {
    // Arrange
    let facts = moving_back(vec![
        back("crypto-common", "0.1.7", "0.1.6", false),
        back("memchr", "2.7.0", "2.5.0", false),
    ]);

    // Act
    let decision = MovedBack.applied(decided(Outcome::Ripe, false), &facts);

    // Assert
    assert_eq!(
        decision,
        Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(10),
            suggestion: Some(Suggestion::MovesBack),
            reasons: vec![
                String::from("compatible"),
                String::from("moves crypto-common back from 0.1.7 to 0.1.6"),
                String::from("moves memchr back from 2.7.0 to 2.5.0"),
            ],
            why: Some(String::from(
                "it moves crypto-common back from 0.1.7 to 0.1.6, memchr back from 2.7.0 to 2.5.0, and a move to a lower version is a person's call"
            )),
        }
    );
}

#[test]
fn applied_to_a_unit_moving_back_off_a_yanked_release_names_the_cure_and_keeps_its_outcome() {
    // Arrange
    let facts = moving_back(vec![back("crypto-common", "0.1.7", "0.1.6", true)]);

    // Act
    let decision = MovedBack.applied(decided(Outcome::Ripe, true), &facts);

    // Assert
    assert_eq!(
        decision,
        Decision {
            reasons: vec![
                String::from("compatible"),
                String::from("moves crypto-common back from the yanked 0.1.7 to 0.1.6"),
            ],
            ..decided(Outcome::Ripe, true)
        }
    );
}

#[test]
fn applied_to_a_unit_moving_nothing_back_changes_nothing() {
    // Arrange
    let facts = moving_back(Vec::new());

    // Act
    let decision = MovedBack.applied(decided(Outcome::Ripe, true), &facts);

    // Assert
    assert_eq!(decision, decided(Outcome::Ripe, true));
}

#[test]
fn applied_to_an_asking_or_ripening_unit_names_the_move_and_keeps_its_outcome() {
    // Arrange
    let facts = moving_back(vec![back("crypto-common", "0.1.7", "0.1.6", false)]);

    // Act
    let decisions = [Outcome::Ask, Outcome::Ripening]
        .map(|outcome| MovedBack.applied(decided(outcome, false), &facts));

    // Assert
    assert_eq!(
        decisions.map(|decision| (decision.outcome, decision.suggestion, decision.reasons)),
        [Outcome::Ask, Outcome::Ripening].map(|outcome| (
            outcome,
            None,
            vec![
                String::from("compatible"),
                String::from("moves crypto-common back from 0.1.7 to 0.1.6"),
            ]
        ))
    );
}

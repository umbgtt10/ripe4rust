// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::releases::version;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::dependent::Dependent;
use ripe4rust_core::planning::judging::held_by_dependents::HeldByDependents;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::requirement::Requirement;

const HELD: &str = "tonic 0.12.3 requires prost ^0.13: moving it adds 0.14.6 beside 0.13.5";

fn decided(outcome: Outcome, urgent: bool, rule: u8) -> Decision {
    Decision {
        outcome,
        urgent,
        rule: Some(rule),
        suggestion: None,
        reasons: vec![String::from("footprint 3 lines in 1 file")],
        why: None,
    }
}

fn held(dependents: &[(&str, &str, &str)]) -> UnitFacts {
    UnitFacts {
        name: String::from("prost"),
        held_by: dependents
            .iter()
            .map(|(name, at, requirement)| Dependent {
                name: String::from(*name),
                version: version(at),
                requires: String::from("prost"),
                requirement: Requirement::parse(requirement).expect("a requirement"),
                in_use: version("0.13.5"),
                target: version("0.14.6"),
            })
            .collect(),
        ..facts(UnitKind::Breaking)
    }
}

#[test]
fn applied_names_every_dependent_of_the_crate_in_one_reason() {
    // Arrange
    let facts = held(&[
        ("prost-build", "0.13.5", "^0.13.5"),
        ("tonic", "0.12.3", "^0.13"),
    ]);

    // Act
    let decision = HeldByDependents.applied(decided(Outcome::Ripe, false, 11), &facts);

    // Assert
    assert_eq!(
        decision.reasons[1],
        "prost-build 0.13.5 requires prost ^0.13.5, tonic 0.12.3 requires prost ^0.13: moving it adds 0.14.6 beside 0.13.5"
    );
}

#[test]
fn applied_to_a_breaking_unit_the_rules_would_ask_ripens_it() {
    // Arrange
    let facts = held(&[("tonic", "0.12.3", "^0.13")]);
    let asked = Decision {
        suggestion: Some(Suggestion::MigrateNow),
        why: Some(String::from("it is a migration")),
        ..decided(Outcome::Ask, false, 12)
    };

    // Act
    let decision = HeldByDependents.applied(asked, &facts);

    // Assert
    assert_eq!(
        (
            decision.outcome,
            decision.rule,
            decision.suggestion,
            decision.why
        ),
        (
            Outcome::Ripening,
            Some(12),
            Some(Suggestion::UntilItsDependentsMove),
            None
        )
    );
}

#[test]
fn applied_to_a_breaking_unit_the_rules_would_take_ripens_it_naming_the_dependent() {
    // Arrange
    let facts = held(&[("tonic", "0.12.3", "^0.13")]);

    // Act
    let decision = HeldByDependents.applied(decided(Outcome::Ripe, false, 11), &facts);

    // Assert
    assert_eq!(
        decision,
        Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(11),
            suggestion: Some(Suggestion::UntilItsDependentsMove),
            reasons: vec![
                String::from("footprint 3 lines in 1 file"),
                String::from(HELD)
            ],
            why: None,
        }
    );
}

#[test]
fn applied_to_a_unit_already_ripening_names_the_dependent_and_keeps_what_holds_it() {
    // Arrange
    let facts = held(&[("tonic", "0.12.3", "^0.13")]);
    let young = Decision {
        suggestion: Some(Suggestion::UntilItSettles),
        ..decided(Outcome::Ripening, false, 9)
    };

    // Act
    let decision = HeldByDependents.applied(young.clone(), &facts);

    // Assert
    assert_eq!(
        decision,
        Decision {
            reasons: vec![
                String::from("footprint 3 lines in 1 file"),
                String::from(HELD)
            ],
            ..young
        }
    );
}

#[test]
fn applied_to_an_urgent_unit_names_the_dependent_and_keeps_it_asked() {
    // Arrange
    let facts = held(&[("tonic", "0.12.3", "^0.13")]);
    let urgent = Decision {
        suggestion: Some(Suggestion::MigrateNow),
        ..decided(Outcome::Ask, true, 3)
    };

    // Act
    let decision = HeldByDependents.applied(urgent.clone(), &facts);

    // Assert
    assert_eq!(
        decision,
        Decision {
            reasons: vec![
                String::from("footprint 3 lines in 1 file"),
                String::from(HELD)
            ],
            ..urgent
        }
    );
}

#[test]
fn applied_with_no_dependent_holding_the_crate_changes_nothing() {
    // Arrange
    let facts = held(&[]);

    // Act
    let decision = HeldByDependents.applied(decided(Outcome::Ripe, false, 11), &facts);

    // Assert
    assert_eq!(decision, decided(Outcome::Ripe, false, 11));
}

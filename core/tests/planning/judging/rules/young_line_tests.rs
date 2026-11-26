// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::standing;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::young_line::YoungLine;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_compatible_unit_is_none() {
    // Arrange
    let young = UnitFacts {
        new_line: Some(standing(2, true, true)),
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = YoungLine.decide(&young);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_settled_new_line_is_none() {
    // Arrange & Act
    let decision = YoungLine.decide(&facts(UnitKind::Toolchain));

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_young_new_line_is_ripening_until_it_settles() {
    // Arrange
    let young = UnitFacts {
        new_line: Some(standing(2, true, true)),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = YoungLine.decide(&young);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(9),
            suggestion: Some(Suggestion::UntilItSettles),
            reasons: vec![String::from(
                "2.x is young: 1 release, the first on 2026-11-20"
            )],
            why: None,
        })
    );
}

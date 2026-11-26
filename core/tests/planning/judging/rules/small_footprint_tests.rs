// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::rules::small_footprint::SmallFootprint;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_unit_at_the_small_footprint_is_none() {
    // Arrange
    let at = UnitFacts {
        footprint: Some(Footprint {
            lines: 10,
            files: 2,
            complete: true,
        }),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = SmallFootprint.decide(&at);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_breaking_unit_named_nowhere_is_none() {
    // Arrange
    let nowhere = UnitFacts {
        footprint: Some(Footprint {
            lines: 0,
            files: 0,
            complete: true,
        }),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = SmallFootprint.decide(&nowhere);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_breaking_unit_under_the_small_footprint_is_ripe() {
    // Arrange
    let small = UnitFacts {
        footprint: Some(Footprint {
            lines: 9,
            files: 2,
            complete: true,
        }),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = SmallFootprint.decide(&small);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(11),
            suggestion: None,
            reasons: vec![String::from("footprint 9 lines in 2 files")],
            why: None,
        })
    );
}

#[test]
fn decide_of_a_breaking_unit_whose_footprint_is_a_floor_is_none() {
    // Arrange
    let floor = UnitFacts {
        footprint: Some(Footprint {
            lines: 2,
            files: 1,
            complete: false,
        }),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = SmallFootprint.decide(&floor);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_breaking_unit_whose_footprint_is_unknown_is_none() {
    // Arrange
    let unknown = UnitFacts {
        footprint: None,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = SmallFootprint.decide(&unknown);

    // Assert
    assert_eq!(decision, None);
}

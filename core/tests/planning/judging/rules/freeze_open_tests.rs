// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::releases::date;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::freeze_open::FreezeOpen;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_in_an_open_freeze_is_ripening_until_it_closes() {
    // Arrange
    let frozen = UnitFacts {
        freeze: Some(date("2027-01-15")),
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = FreezeOpen.decide(&frozen);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(6),
            suggestion: Some(Suggestion::UntilTheWindowCloses),
            reasons: vec![String::from("frozen until 2027-01-15")],
            why: None,
        })
    );
}

#[test]
fn decide_with_no_freeze_open_is_none() {
    // Arrange & Act
    let decision = FreezeOpen.decide(&facts(UnitKind::Compatible));

    // Assert
    assert_eq!(decision, None);
}

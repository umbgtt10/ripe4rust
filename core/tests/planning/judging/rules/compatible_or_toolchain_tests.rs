// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::compatible_or_toolchain::CompatibleOrToolchain;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_breaking_unit_is_none() {
    // Arrange & Act
    let decision = CompatibleOrToolchain.decide(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_compatible_unit_is_ripe() {
    // Arrange & Act
    let decision = CompatibleOrToolchain.decide(&facts(UnitKind::Compatible));

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ripe,
            urgent: false,
            rule: Some(10),
            suggestion: None,
            reasons: vec![String::from("compatible")],
            why: None,
        })
    );
}

#[test]
fn decide_of_the_rest_counts_its_moves() {
    // Arrange
    let rest = UnitFacts {
        rest: Some(2),
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = CompatibleOrToolchain.decide(&rest);

    // Assert
    assert_eq!(
        decision.map(|decision| decision.reasons),
        Some(vec![String::from("2 moves no unit reaches")])
    );
}

#[test]
fn decide_of_the_toolchain_counts_what_it_releases() {
    // Arrange
    let toolchain = UnitFacts {
        held_back: 2,
        ..facts(UnitKind::Toolchain)
    };

    // Act
    let decision = CompatibleOrToolchain.decide(&toolchain);

    // Assert
    assert_eq!(
        decision.map(|decision| decision.reasons),
        Some(vec![
            String::from("toolchain"),
            String::from("releases 2 versions held back"),
        ])
    );
}

#[test]
fn decide_of_the_toolchain_releasing_nothing_says_toolchain() {
    // Arrange & Act
    let decision = CompatibleOrToolchain.decide(&facts(UnitKind::Toolchain));

    // Assert
    assert_eq!(
        decision.map(|decision| decision.reasons),
        Some(vec![String::from("toolchain")])
    );
}

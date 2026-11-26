// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::rules::git_unit::GitUnit;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn decide_of_a_git_unit_asks_it_a_person_deciding() {
    // Arrange & Act
    let decision = GitUnit.decide(&facts(UnitKind::Git));

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(13),
            suggestion: None,
            reasons: vec![String::from(
                "git source: no version promise, a person decides"
            )],
            why: Some(String::from(
                "it moves a git source, which makes no version promise: a person decides whether to take it"
            )),
        })
    );
}

#[test]
fn decide_of_any_other_unit_is_none() {
    // Arrange & Act
    let decisions = [
        UnitKind::Compatible,
        UnitKind::Breaking,
        UnitKind::Toolchain,
    ]
    .map(|kind| GitUnit.decide(&facts(kind)));

    // Assert
    assert_eq!(decisions, [None, None, None]);
}

#[test]
fn reason_says_a_git_source_makes_no_version_promise() {
    // Arrange & Act
    let reason = GitUnit.reason();

    // Assert
    assert_eq!(reason, "git source: no version promise, a person decides");
}

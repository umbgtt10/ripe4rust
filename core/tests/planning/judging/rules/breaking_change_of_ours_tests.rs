// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::raise::Raise;
use ripe4rust_core::planning::judging::rules::breaking_change_of_ours::BreakingChangeOfOurs;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::traits::rule::Rule;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::partial_version::PartialVersion;

fn raise(published: bool) -> Raise {
    Raise {
        member: String::from("widget-core"),
        to: PartialVersion::parse("1.88").expect("a rust-version"),
        published,
    }
}

#[test]
fn decide_of_a_compatible_unit_is_none() {
    // Arrange
    let exposed = UnitFacts {
        exposed_in: vec![String::from("widget-core")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&exposed);

    // Assert
    assert_eq!(decision, None);
}

#[test]
fn decide_of_a_crate_a_published_member_uses_unscanned_asks_it() {
    // Arrange
    let unknown = UnitFacts {
        exposure_unknown_in: vec![String::from("widget-core")],
        footprint: None,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&unknown);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(8),
            suggestion: Some(Suggestion::BreakingChangeOfOurs),
            reasons: vec![
                String::from("exposure in widget-core's public API not yet known"),
                String::from("wait: footprint not yet known"),
            ],
            why: Some(String::from(
                "whether widget's types are in widget-core's public API is not yet known, and if they are, the upgrade is a breaking change of ours too"
            )),
        })
    );
}

#[test]
fn decide_of_a_crate_both_exposed_and_unscanned_says_it_is_ours() {
    // Arrange
    let both = UnitFacts {
        exposed_in: vec![String::from("widget-core")],
        exposure_unknown_in: vec![String::from("widget-tools")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&both);

    // Assert
    assert_eq!(
        decision.and_then(|decision| decision.why),
        Some(String::from(
            "widget's types are in widget-core's public API; whether widget's types are in widget-tools's public API is not yet known: the upgrade is a breaking change of ours too"
        ))
    );
}

#[test]
fn decide_of_a_crate_in_a_published_api_asks_it() {
    // Arrange
    let exposed = UnitFacts {
        exposed_in: vec![String::from("widget-core")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&exposed);

    // Assert
    assert_eq!(
        decision,
        Some(Decision {
            outcome: Outcome::Ask,
            urgent: false,
            rule: Some(8),
            suggestion: Some(Suggestion::BreakingChangeOfOurs),
            reasons: vec![
                String::from("in widget-core's public API"),
                String::from("plan it: footprint 42 lines in 7 files"),
            ],
            why: Some(String::from(
                "widget's types are in widget-core's public API: the upgrade is a breaking change of ours too"
            )),
        })
    );
}

#[test]
fn decide_of_a_raise_of_a_published_rust_version_asks_it() {
    // Arrange
    let raising = UnitFacts {
        raises: vec![raise(true)],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&raising);

    // Assert
    assert_eq!(
        decision.and_then(|decision| decision.why),
        Some(String::from(
            "it raises widget-core's rust-version to 1.88: the upgrade is a breaking change of ours too"
        ))
    );
}

#[test]
fn decide_of_a_raise_of_an_unpublished_rust_version_is_none() {
    // Arrange
    let raising = UnitFacts {
        raises: vec![raise(false)],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = BreakingChangeOfOurs.decide(&raising);

    // Assert
    assert_eq!(decision, None);
}

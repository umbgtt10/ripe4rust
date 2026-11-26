// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::named_nowhere::NamedNowhere;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

const NOWHERE: &str = "named nowhere in the sources: possibly an unused dependency";

fn asked(reasons: &[&str]) -> Decision {
    Decision {
        outcome: Outcome::Ask,
        urgent: true,
        rule: Some(3),
        suggestion: None,
        reasons: reasons.iter().map(|reason| String::from(*reason)).collect(),
        why: None,
    }
}

fn nowhere(kind: UnitKind, complete: bool) -> UnitFacts {
    UnitFacts {
        footprint: Some(Footprint {
            lines: 0,
            files: 0,
            complete,
        }),
        ..facts(kind)
    }
}

#[test]
fn applied_to_a_breaking_unit_named_nowhere_adds_that_it_may_be_unused() {
    // Arrange
    let facts = nowhere(UnitKind::Breaking, true);

    // Act
    let decision = NamedNowhere.applied(asked(&["clears RUSTSEC-2026-0097", "breaking"]), &facts);

    // Assert
    assert_eq!(
        decision,
        asked(&["clears RUSTSEC-2026-0097", "breaking", NOWHERE])
    );
}

#[test]
fn applied_to_a_unit_named_somewhere_or_not_known_or_not_breaking_adds_nothing() {
    // Arrange
    let named = UnitFacts {
        footprint: Some(Footprint {
            lines: 4,
            files: 1,
            complete: true,
        }),
        ..facts(UnitKind::Breaking)
    };
    let all = [
        named,
        nowhere(UnitKind::Breaking, false),
        nowhere(UnitKind::Compatible, true),
        UnitFacts {
            footprint: None,
            ..facts(UnitKind::Breaking)
        },
    ];

    // Act
    let decisions = all.map(|facts| NamedNowhere.applied(asked(&["breaking"]), &facts));

    // Assert
    assert_eq!(decisions, [0, 1, 2, 3].map(|_| asked(&["breaking"])));
}

#[test]
fn applied_to_a_unit_whose_reasons_say_so_already_adds_nothing() {
    // Arrange
    let facts = nowhere(UnitKind::Breaking, true);
    let said = format!("migrate now: {NOWHERE}");

    // Act
    let decision = NamedNowhere.applied(asked(&[&said]), &facts);

    // Assert
    assert_eq!(decision, asked(&[&said]));
}

#[test]
fn fits_a_breaking_unit_a_complete_parse_finds_named_nowhere_alone() {
    // Arrange
    let all = [
        nowhere(UnitKind::Breaking, true),
        nowhere(UnitKind::Breaking, false),
        nowhere(UnitKind::Compatible, true),
        UnitFacts {
            footprint: None,
            ..facts(UnitKind::Breaking)
        },
    ];

    // Act
    let fits = all.map(|facts| NamedNowhere.fits(&facts));

    // Assert
    assert_eq!(fits, [true, false, false, false]);
}

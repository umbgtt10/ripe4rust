// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use crate::support::facts::standing;
use crate::support::releases::date;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::judge::Judge;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

fn rule_of(facts: &UnitFacts) -> Option<u8> {
    Judge.judge(facts).rule
}

#[test]
fn judge_of_a_plain_breaking_unit_is_rule_twelve_with_the_verdict() {
    // Arrange & Act
    let decision = Judge.judge(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(decision.rule, Some(12));
    assert_eq!(decision.suggestion, Some(Suggestion::PlanIt));
}

#[test]
fn judge_of_a_plain_compatible_unit_is_rule_ten_ripe() {
    // Arrange & Act
    let decision = Judge.judge(&facts(UnitKind::Compatible));

    // Assert
    assert_eq!(decision.outcome, Outcome::Ripe);
    assert_eq!(decision.rule, Some(10));
}

#[test]
fn judge_of_a_ripe_unit_without_gates_asks_it() {
    // Arrange
    let ungated = UnitFacts {
        gates: 0,
        ..facts(UnitKind::Toolchain)
    };

    // Act
    let decision = Judge.judge(&ungated);

    // Assert
    assert_eq!(decision.outcome, Outcome::Ask);
    assert_eq!(decision.rule, Some(10));
    assert_eq!(decision.suggestion, Some(Suggestion::DeclareTheGates));
}

#[test]
fn judge_of_a_young_breaking_line_far_behind_asks_it_overripe() {
    // Arrange
    let behind = UnitFacts {
        new_line: Some(standing(4, true, true)),
        lines_behind: 3,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let decision = Judge.judge(&behind);

    // Assert
    assert_eq!(decision.outcome, Outcome::Ask);
    assert_eq!(decision.rule, Some(9));
    assert_eq!(decision.suggestion, Some(Suggestion::Overripe));
}

#[test]
fn judge_of_an_advisory_row_asks_it_urgently_by_no_rule() {
    // Arrange
    let row = UnitFacts {
        name: String::from("url 2.5.8"),
        concerns: vec![String::from("RUSTSEC-2026-0002")],
        ..facts(UnitKind::Advisory)
    };

    // Act
    let decision = Judge.judge(&row);

    // Assert
    assert_eq!(
        decision,
        Decision {
            outcome: Outcome::Ask,
            urgent: true,
            rule: None,
            suggestion: Some(Suggestion::ReplaceOrAccept),
            reasons: vec![String::from("RUSTSEC-2026-0002, which no upgrade clears")],
            why: Some(String::from(
                "RUSTSEC-2026-0002 affects url 2.5.8, and no upgrade in this plan clears it"
            )),
        }
    );
}

#[test]
fn judge_of_breaking_units_runs_rules_eight_to_twelve_in_order() {
    // Arrange
    let breaking = UnitFacts {
        exposed_in: vec![String::from("widget-core")],
        new_line: Some(standing(2, true, true)),
        footprint: Footprint { lines: 3, files: 1 },
        ..facts(UnitKind::Breaking)
    };

    // Act & Assert
    assert_eq!(rule_of(&breaking), Some(8));
    assert_eq!(
        rule_of(&UnitFacts {
            exposed_in: Vec::new(),
            ..breaking.clone()
        }),
        Some(9)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            exposed_in: Vec::new(),
            new_line: Some(standing(2, false, true)),
            ..breaking.clone()
        }),
        Some(11)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            concerns: vec![String::from("RUSTSEC-2026-0190")],
            ..breaking
        }),
        Some(3)
    );
}

#[test]
fn judge_takes_the_first_rule_that_matches() {
    // Arrange
    let everything = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        pins: vec![pinned(None)],
        fully_held: true,
        freeze: Some(date("2027-01-15")),
        ..facts(UnitKind::Compatible)
    };

    // Act & Assert
    assert_eq!(rule_of(&everything), Some(1));
    assert_eq!(
        rule_of(&UnitFacts {
            reaches: Vec::new(),
            ..everything.clone()
        }),
        Some(2)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            concerns: Vec::new(),
            ..everything.clone()
        }),
        Some(4)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            concerns: Vec::new(),
            pins: vec![pinned(Some("review 2027-01-31 reached"))],
            ..everything.clone()
        }),
        Some(5)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            concerns: Vec::new(),
            pins: Vec::new(),
            ..everything.clone()
        }),
        Some(6)
    );
    assert_eq!(
        rule_of(&UnitFacts {
            concerns: Vec::new(),
            pins: Vec::new(),
            freeze: None,
            ..everything
        }),
        Some(7)
    );
}

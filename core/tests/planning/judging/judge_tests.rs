// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use crate::support::facts::standing;
use crate::support::releases::date;
use crate::support::releases::version;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::decision::Decision;
use ripe4rust_core::planning::judging::downgrade::Downgrade;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::judge::Judge;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::unit_kind::UnitKind;

fn rule_of(facts: &UnitFacts) -> Option<u8> {
    Judge.judge(facts).rule
}

#[test]
fn judge_of_a_breaking_unit_named_nowhere_asks_it_saying_it_may_be_unused() {
    // Arrange
    let nowhere = UnitFacts {
        footprint: Some(Footprint {
            lines: 0,
            files: 0,
            complete: true,
        }),
        ..facts(UnitKind::Breaking)
    };
    let young = UnitFacts {
        new_line: Some(standing(2, true, true)),
        lines_behind: 0,
        ..nowhere.clone()
    };

    // Act
    let decisions = [Judge.judge(&nowhere), Judge.judge(&young)];

    // Assert
    assert_eq!(
        decisions.map(|decision| (decision.outcome, decision.rule, decision.reasons)),
        [
            (
                Outcome::Ask,
                Some(12),
                vec![String::from(
                    "migrate now: named nowhere in the sources: possibly an unused dependency"
                )]
            ),
            (
                Outcome::Ripening,
                Some(9),
                vec![
                    String::from("2.x is young: 1 release, the first on 2026-11-20"),
                    String::from("named nowhere in the sources: possibly an unused dependency")
                ]
            ),
        ]
    );
}

#[test]
fn judge_of_a_git_unit_asks_it_held_by_a_pin_and_urgent_where_it_clears() {
    // Arrange
    let plain = facts(UnitKind::Git);
    let urgent = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0097")],
        ..facts(UnitKind::Git)
    };
    let pinned = UnitFacts {
        pins: vec![pinned(None)],
        fully_held: true,
        ..facts(UnitKind::Git)
    };

    // Act
    let decisions = [plain, urgent, pinned].map(|facts| Judge.judge(&facts));

    // Assert
    assert_eq!(
        decisions.map(|decision| (decision.outcome, decision.urgent, decision.rule)),
        [
            (Outcome::Ask, false, Some(13)),
            (Outcome::Ask, true, Some(3)),
            (Outcome::Ripening, false, Some(4)),
        ]
    );
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
fn judge_of_a_ripe_unit_moving_a_crate_back_asks_it_and_without_gates_asks_for_them() {
    // Arrange
    let downgrades = vec![Downgrade {
        name: String::from("crypto-common"),
        from: version("0.1.7"),
        to: version("0.1.6"),
        yanked: false,
    }];
    let gated = UnitFacts {
        downgrades: downgrades.clone(),
        ..facts(UnitKind::Compatible)
    };
    let ungated = UnitFacts {
        gates: 0,
        ..gated.clone()
    };

    // Act
    let decisions = [Judge.judge(&gated), Judge.judge(&ungated)];

    // Assert
    assert_eq!(
        decisions.map(|decision| (decision.outcome, decision.suggestion, decision.reasons)),
        [
            (
                Outcome::Ask,
                Some(Suggestion::MovesBack),
                vec![
                    String::from("compatible"),
                    String::from("moves crypto-common back from 0.1.7 to 0.1.6")
                ]
            ),
            (
                Outcome::Ask,
                Some(Suggestion::DeclareTheGates),
                vec![
                    String::from("compatible"),
                    String::from("no gates declared"),
                    String::from("moves crypto-common back from 0.1.7 to 0.1.6")
                ]
            ),
        ]
    );
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
        footprint: Some(Footprint {
            lines: 3,
            files: 1,
            complete: true,
        }),
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

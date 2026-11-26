// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::standing;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::suggestion::Suggestion;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::planning::judging::verdict::Verdict;
use ripe4rust_core::planning::judging::verdict_call::VerdictCall;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn call_of_a_large_footprint_is_wait() {
    // Arrange
    let large = UnitFacts {
        footprint: Footprint {
            lines: 100,
            files: 12,
        },
        ..facts(UnitKind::Breaking)
    };

    // Act
    let call = Verdict.call(&large);

    // Assert
    assert_eq!(
        call,
        VerdictCall {
            suggestion: Suggestion::Wait,
            reason: String::from("footprint 100 lines in 12 files, at or above 100"),
        }
    );
}

#[test]
fn call_of_a_migration_removing_a_second_copy_is_migrate_now() {
    // Arrange
    let copy = UnitFacts {
        removes_copy: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let call = Verdict.call(&copy);

    // Assert
    assert_eq!(
        call,
        VerdictCall {
            suggestion: Suggestion::MigrateNow,
            reason: String::from("it removes a second copy of widget from the tree"),
        }
    );
}

#[test]
fn call_of_a_settled_line_and_a_moderate_footprint_is_plan_it() {
    // Arrange & Act
    let call = Verdict.call(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(
        call,
        VerdictCall {
            suggestion: Suggestion::PlanIt,
            reason: String::from("footprint 42 lines in 7 files"),
        }
    );
}

#[test]
fn call_of_a_stale_line_against_an_active_one_is_migrate_now() {
    // Arrange
    let stale = UnitFacts {
        in_use_line: Some(standing(1, false, false)),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let call = Verdict.call(&stale);

    // Assert
    assert_eq!(
        call,
        VerdictCall {
            suggestion: Suggestion::MigrateNow,
            reason: String::from("1.x is stale while 2.x is active"),
        }
    );
}

#[test]
fn call_of_a_young_line_is_wait() {
    // Arrange
    let young = UnitFacts {
        new_line: Some(standing(2, true, true)),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let call = Verdict.call(&young);

    // Assert
    assert_eq!(
        call,
        VerdictCall {
            suggestion: Suggestion::Wait,
            reason: String::from("2.x is young"),
        }
    );
}

#[test]
fn reason_writes_the_call_s_words_and_its_reason() {
    // Arrange & Act
    let reason = Verdict.reason(&facts(UnitKind::Breaking));

    // Assert
    assert_eq!(reason, "plan it: footprint 42 lines in 7 files");
}

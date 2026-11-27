// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::command::exits::Exits;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

fn failure(code: DiagnosticCode) -> PlanFailure {
    PlanFailure {
        diagnostics: vec![
            DiagnosticCode::GatesUndeclared.diagnose(None, String::from("no gates"), String::new()),
            code.diagnose(None, String::from("what"), String::from("do")),
        ],
    }
}

fn report(as_planned: bool) -> CheckReport {
    CheckReport {
        unit: String::from("compatible:itoa"),
        as_planned,
        transitive: Vec::new(),
        differences: Vec::new(),
        shared: Vec::new(),
        applied: Vec::new(),
        to_go: Vec::new(),
        diagnostics: Vec::new(),
    }
}

#[test]
fn check_of_a_unit_as_planned_is_zero_and_otherwise_one() {
    // Arrange & Act
    let codes = [Exits.check(&report(true)), Exits.check(&report(false))];

    // Assert
    assert_eq!(codes, [0, 1]);
}

#[test]
fn failure_of_a_plan_stale_is_three_and_of_anything_else_two() {
    // Arrange & Act
    let codes = [
        Exits.failure(&failure(DiagnosticCode::PlanStale)),
        Exits.failure(&failure(DiagnosticCode::PlanUnreadable)),
    ];

    // Assert
    assert_eq!(codes, [3, 2]);
}

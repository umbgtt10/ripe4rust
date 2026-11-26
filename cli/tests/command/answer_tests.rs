// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::compatible;
use crate::support::plans::plan;
use crate::support::reports::as_planned;
use crate::support::reports::held;
use crate::support::reports::not_as_planned;
use cargo_ripe4rust::command::answer::Answer;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::ran::Ran;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::guiding::guide::Guide;

fn stale() -> PlanFailure {
    PlanFailure {
        diagnostics: vec![DiagnosticCode::PlanStale.diagnose(
            None,
            String::from("the lock moved"),
            String::from("plan again"),
        )],
    }
}

#[test]
fn checked_of_a_report_exits_by_whether_it_is_as_planned() {
    // Arrange
    let answer = Answer::new(Format::Table, "check");

    // Act
    let codes = [
        answer.checked(Ok(as_planned())).code,
        answer.checked(Ok(not_as_planned())).code,
        answer.checked(Err(stale())).code,
    ];

    // Assert
    assert_eq!(codes, [0, 1, 3]);
}

#[test]
fn failed_prints_the_failure_and_exits_by_its_diagnostics() {
    // Arrange & Act
    let ran = Answer::new(Format::Table, "check").failed(&stale());

    // Assert
    assert_eq!(
        ran,
        Ran {
            output: String::from("error[plan-stale] the lock moved. Do: plan again\n"),
            code: 3,
        }
    );
}

#[test]
fn guided_of_the_guide_exits_zero() {
    // Arrange & Act
    let ran = Answer::new(Format::Json, "guide").guided(&Guide::new("0.0.1").text());

    // Assert
    assert_eq!(ran.code, 0);
    assert!(
        ran.output.contains("\"version\": \"0.0.1\""),
        "{}",
        ran.output
    );
}

#[test]
fn held_of_pins_printed_exits_zero_and_of_a_stale_plan_three() {
    // Arrange
    let answer = Answer::new(Format::Table, "hold");

    // Act
    let codes = [
        answer.held(Ok(held(true))).code,
        answer.held(Err(stale())).code,
    ];

    // Assert
    assert_eq!(codes, [0, 3]);
}

#[test]
fn planned_of_a_plan_exits_zero_whatever_it_says() {
    // Arrange & Act
    let ran = Answer::new(Format::Json, "plan").planned(Ok(plan(vec![compatible()])));

    // Assert
    assert_eq!(ran.code, 0);
    assert!(
        ran.output.starts_with("{\n  \"schema\": 1,"),
        "{}",
        ran.output
    );
}

#[test]
fn versioned_prints_the_binary_and_its_release_in_either_format_and_exits_zero() {
    // Arrange & Act
    let ran = [
        Answer::new(Format::Table, "--version").versioned("9.8.7"),
        Answer::new(Format::Json, "--version").versioned("9.8.7"),
    ];

    // Assert
    assert_eq!(
        ran,
        [
            Ran {
                output: String::from("cargo-ripe4rust 9.8.7\n"),
                code: 0
            },
            Ran {
                output: String::from("cargo-ripe4rust 9.8.7\n"),
                code: 0
            }
        ]
    );
}

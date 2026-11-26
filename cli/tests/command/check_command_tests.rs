// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::command::check_command::CheckCommand;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::request::Request;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const PLAN: &str = include_str!("../fixtures/workspace/plan.json");

fn request(plan: &str) -> Request {
    Request {
        command: String::from("check"),
        format: Format::Json,
        date: None,
        offline: false,
        unit: Some(String::from("compatible:itoa")),
        plan: Some(String::from(plan)),
        because: None,
        review: None,
    }
}

#[test]
fn check_of_a_plan_at_an_absolute_path_reads_it_there() {
    // Arrange
    let runner = layout();
    let files = files().with("/plans/today.json", PLAN);
    let (pause, clock) = (
        CountingPause::new(),
        FixedClock::new(Date::parse("2026-11-26").expect("a date")),
    );
    let seams = Seams {
        runner: &runner,
        files: &files,
        pause: &pause,
        clock: &clock,
        dir: PathBuf::from("/work/ws"),
        cache: None,
    };

    // Act
    let report = CheckCommand::new(seams).check(&request("/plans/today.json"));

    // Assert
    assert_eq!(report.map(|report| report.as_planned), Ok(false));
}

#[test]
fn check_of_a_plan_named_from_where_it_runs_reads_it_there() {
    // Arrange
    let runner = layout();
    let files = files();
    let (pause, clock) = (
        CountingPause::new(),
        FixedClock::new(Date::parse("2026-11-26").expect("a date")),
    );
    let seams = Seams {
        runner: &runner,
        files: &files,
        pause: &pause,
        clock: &clock,
        dir: PathBuf::from("/work/ws"),
        cache: None,
    };

    // Act
    let failure = CheckCommand::new(seams)
        .check(&request("plan.json"))
        .expect_err("no plan there");

    // Assert
    assert_eq!(
        (
            failure.diagnostics[0].code,
            failure.diagnostics[0].what.as_str()
        ),
        (
            DiagnosticCode::PlanUnreadable,
            "`plan.json` could not be read as a plan: it is not there"
        )
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::hold_command::HoldCommand;
use cargo_ripe4rust::command::request::Request;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const PLAN: &str = include_str!("../fixtures/workspace/plan.json");

#[test]
fn hold_of_a_unit_takes_the_reason_and_review_the_line_gave() {
    // Arrange
    let runner = layout();
    let files = files().with("/work/ws/plan.json", PLAN);
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
    let request = Request {
        command: String::from("hold"),
        format: Format::Json,
        date: None,
        offline: false,
        unit: Some(String::from("compatible:memchr")),
        plan: Some(String::from("plan.json")),
        because: Some(String::from("2.8.3 broke the gates")),
        review: Some(Date::parse("2027-01-31").expect("a date")),
    };

    // Act
    let report = HoldCommand::new(seams).hold(&request).expect("a pin");

    // Assert
    assert_eq!(
        report.pins[0].line,
        r#"memchr = { reason = "2.8.3 broke the gates", review = "2027-01-31", hold = "all" }"#
    );
}

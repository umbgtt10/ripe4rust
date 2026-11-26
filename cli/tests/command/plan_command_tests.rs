// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::recorded::recorded;
use crate::support::workspace::files;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::plan_command::PlanCommand;
use cargo_ripe4rust::command::request::Request;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

fn date(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

fn request(date: Option<Date>) -> Request {
    Request {
        command: String::from("plan"),
        format: Format::Json,
        date,
        offline: false,
        unit: None,
        plan: None,
        because: None,
        review: None,
    }
}

#[test]
fn plan_of_the_recorded_workspace_is_dated_the_day_asked() {
    // Arrange
    let (runner, files, pause) = (recorded(false), files(), CountingPause::new());
    let clock = FixedClock::new(date("2026-12-01"));
    let seams = Seams {
        runner: &runner,
        files: &files,
        pause: &pause,
        clock: &clock,
        dir: PathBuf::from("/work/ws"),
        cache: Some(PathBuf::from("/cache")),
    };

    // Act
    let plan = PlanCommand::new(seams)
        .plan(&request(Some(date("2026-11-26"))))
        .expect("a plan");

    // Assert
    assert_eq!(
        (plan.date, plan.registry_read),
        (date("2026-11-26"), date("2026-11-26"))
    );
    assert!(files.contents("/cache/2026-11-26/index/smallvec").is_some());
}

#[test]
fn plan_without_a_date_asked_is_dated_today() {
    // Arrange
    let (runner, files, pause) = (recorded(false), files(), CountingPause::new());
    let clock = FixedClock::new(date("2026-11-27"));
    let seams = Seams {
        runner: &runner,
        files: &files,
        pause: &pause,
        clock: &clock,
        dir: PathBuf::from("/work/ws"),
        cache: Some(PathBuf::from("/cache")),
    };

    // Act
    let plan = PlanCommand::new(seams)
        .plan(&request(None))
        .expect("a plan");

    // Assert
    assert_eq!(plan.date, date("2026-11-27"));
    assert_eq!(*pause.pauses.borrow(), vec![1, 1, 1]);
}

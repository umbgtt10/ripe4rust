// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::command::round_adapters::RoundAdapters;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const PLAN: &str = include_str!("../fixtures/workspace/plan.json");

#[test]
fn with_reads_the_plan_named_and_the_workspace_s_files() {
    // Arrange
    let runner = layout();
    let files = files().with("/work/ws/plans/today.json", PLAN);
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
    let (plan, fingerprint) =
        RoundAdapters::new(seams).with("plans/today.json", |source, files| {
            (
                source.plan().map(|plan| plan.fingerprint),
                files.fingerprint(),
            )
        });

    // Assert
    assert_eq!(plan, fingerprint);
}

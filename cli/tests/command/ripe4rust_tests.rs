// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::failed;
use crate::support::recorded::recorded;
use crate::support::replay_runner::ReplayRunner;
use crate::support::workspace::files;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::command::ripe4rust::Ripe4rust;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::values::date::Date;
use std::path::PathBuf;

const JSON: [&str; 5] = ["ripe4rust", "plan", "--format", "json", "--date"];

fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| String::from(*word)).collect()
}

fn clock() -> FixedClock {
    FixedClock::new(Date::parse("2026-11-30").expect("a date"))
}

fn plan_json(date: &str, offline: bool) -> Vec<String> {
    let mut words = args(&JSON);
    words.push(String::from(date));
    if offline {
        words.push(String::from("--offline"));
    }
    words
}

fn seams<'a>(
    runner: &'a ReplayRunner,
    files: &'a MemoryFiles,
    pause: &'a CountingPause,
    clock: &'a FixedClock,
) -> Seams<'a> {
    Seams {
        runner,
        files,
        pause,
        clock,
        dir: PathBuf::from("/work/ws"),
        cache: Some(PathBuf::from("/cache")),
    }
}

#[test]
fn run_of_a_command_not_built_yet_exits_two_saying_so() {
    // Arrange
    let (runner, files, pause, clock) = (
        ReplayRunner::new(),
        MemoryFiles::new(),
        CountingPause::new(),
        clock(),
    );

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&args(&["ripe4rust", "guide"]));

    // Assert
    assert_eq!(ran.code, 2);
    assert_eq!(
        ran.output,
        "error[command-unavailable] `guide` is not in this release of ripe4rust yet. Do: use `cargo ripe4rust plan`, the one command this release runs; `check`, `hold` and `guide` come with a later release\n"
    );
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn run_of_a_policy_in_error_reads_nothing_beyond_it() {
    // Arrange
    let runner = recorded(false);
    let files = files().with(
        "/work/ws/Cargo.toml",
        "[workspace]\nmembers = [\"app\", \"app-core\"]\n\n[workspace.metadata.ripe4rust]\ngates = [\"cargo test\"]\nyoung-dayz = 30\n",
    );
    let (pause, clock) = (CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    assert_eq!(ran.code, 2);
    assert!(
        ran.output.contains("\"code\": \"policy-unknown-key\""),
        "{}",
        ran.output
    );
    assert_eq!(
        runner.commands(),
        vec![String::from("cargo metadata --format-version 1 --no-deps")]
    );
}

#[test]
fn run_of_the_recorded_workspace_prints_the_plan_as_a_table() {
    // Arrange
    let (runner, files, pause, clock) = (recorded(false), files(), CountingPause::new(), clock());

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&args(&[
        "plan",
        "--date",
        "2026-11-26",
    ]));

    // Assert
    assert_eq!(ran.code, 0);
    assert_eq!(ran.output, include_str!("../fixtures/workspace/plan.txt"));
}

#[test]
fn run_of_the_recorded_workspace_prints_the_plan_as_json_and_exits_zero() {
    // Arrange
    let (runner, files, pause, clock) = (recorded(false), files(), CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    assert_eq!(ran.code, 0);
    assert_eq!(ran.output, include_str!("../fixtures/workspace/plan.json"));
}

#[test]
fn run_offline_after_an_online_run_prints_the_same_plan_asking_no_registry() {
    // Arrange
    let (online, files, pause, clock) = (recorded(false), files(), CountingPause::new(), clock());
    let first =
        Ripe4rust::new(seams(&online, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));
    let offline = recorded(true);

    // Act
    let second =
        Ripe4rust::new(seams(&offline, &files, &pause, &clock)).run(&plan_json("2026-11-26", true));

    // Assert
    assert_eq!(second.output, first.output);
    assert!(
        offline
            .commands()
            .iter()
            .all(|command| !command.starts_with("curl")),
        "{:?}",
        offline.commands()
    );
}

#[test]
fn run_offline_on_a_later_day_warns_the_cache_is_old() {
    // Arrange
    let (online, files, pause, clock) = (recorded(false), files(), CountingPause::new(), clock());
    let _ =
        Ripe4rust::new(seams(&online, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));
    let offline = recorded(true);

    // Act
    let ran =
        Ripe4rust::new(seams(&offline, &files, &pause, &clock)).run(&plan_json("2026-11-28", true));

    // Assert
    assert_eq!(ran.code, 0);
    assert!(
        ran.output.contains("\"registry_read\": \"2026-11-26\""),
        "{}",
        ran.output
    );
    assert!(
        ran.output.contains("\"code\": \"registry-cache-old\""),
        "{}",
        ran.output
    );
}

#[test]
fn run_without_cargo_deny_prints_no_plan_and_exits_two() {
    // Arrange
    let runner = recorded(false).reply(
        &[
            "cargo",
            "deny",
            "--format",
            "json",
            "--locked",
            "--workspace",
            "check",
            "--audit-compatible-output",
            "advisories",
        ],
        failed(101, include_str!("../fixtures/cargo-deny/missing.txt")),
    );
    let mut runner = runner;
    let missing = runner.replies.pop().expect("the reply");
    runner.replies.insert(0, missing);
    let (files, pause, clock) = (files(), CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    assert_eq!(ran.code, 2);
    assert!(
        ran.output.contains("\"code\": \"cargo-deny-missing\""),
        "{}",
        ran.output
    );
    assert!(!ran.output.contains("\"units\""), "{}", ran.output);
}

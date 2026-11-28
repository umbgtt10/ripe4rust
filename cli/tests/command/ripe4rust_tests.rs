// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::counting_pause::CountingPause;
use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::failed;
use crate::support::recorded::recorded;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use crate::support::workspace::files;
use crate::support::workspace::layout;
use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use cargo_ripe4rust::adapters::workspace::policy_section::PolicySection;
use cargo_ripe4rust::command::ripe4rust::Ripe4rust;
use cargo_ripe4rust::command::seams::Seams;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::policy_reader::PolicyReader;
use ripe4rust_core::values::date::Date;
use serde_json::Value;
use serde_json::from_str;
use serde_json::json;
use std::path::PathBuf;

const JSON: [&str; 5] = ["ripe4rust", "plan", "--format", "json", "--date"];
const LOCK: &str = include_str!("../fixtures/workspace/files/Cargo.lock.txt");
const PLAN: &str = include_str!("../fixtures/workspace/plan.json");

fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| String::from(*word)).collect()
}

fn check(unit: &str) -> Vec<String> {
    args(&[
        "ripe4rust",
        "check",
        unit,
        "--plan",
        "plan.json",
        "--format",
        "json",
    ])
}

fn clock() -> FixedClock {
    FixedClock::new(Date::parse("2026-11-30").expect("a date"))
}

fn hold(unit: &str, because: &str) -> Vec<String> {
    args(&[
        "ripe4rust",
        "hold",
        unit,
        "--plan",
        "plan.json",
        "--because",
        because,
        "--format",
        "json",
    ])
}

fn moved(lock: &str, name: &str, from: &str, to: &str) -> String {
    lock.replace(
        &format!("name = \"{name}\"\nversion = \"{from}\""),
        &format!("name = \"{name}\"\nversion = \"{to}\""),
    )
}

fn parsed(text: &str) -> Value {
    from_str(text).expect("a JSON document")
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
fn run_of_a_library_that_does_not_parse_warns_where_and_asks_its_exposure() {
    // Arrange
    let runner = recorded(false);
    let files = files().with(
        "/work/ws/app-core/src/lib.rs",
        "pub fn digits(value: u64) -> String {\n    let buffer = ;\n}\n",
    );
    let (pause, clock) = (CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    let document = parsed(&ran.output);
    let itoa = document["units"]
        .as_array()
        .and_then(|units| units.iter().find(|unit| unit["id"] == "breaking:itoa"))
        .expect("the breaking unit");
    assert_eq!(
        (ran.code, &document["diagnostics"][0]),
        (
            0,
            &json!({
                "severity": "warning",
                "code": "source-unparsed",
                "where": "app-core/src/lib.rs:2",
                "what": "`app-core/src/lib.rs` could not be parsed (expected an expression): the footprint of the crates it names may be low",
                "do": "fix the file so it parses, then plan again; or take the footprint as a floor",
                "needs_person": false
            })
        )
    );
    assert_eq!(
        itoa["reasons"][0],
        json!("exposure in app-core's public API not known")
    );
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
fn run_of_a_workspace_on_an_older_toolchain_names_what_the_newest_releases() {
    // Arrange
    let runner = recorded(false);
    let files = files().with(
        "/work/ws/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.62.1\"\n",
    );
    let (pause, clock) = (CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    let document = parsed(&ran.output);
    let toolchain = document["units"]
        .as_array()
        .and_then(|units| units.iter().find(|unit| unit["id"] == "toolchain:rust"))
        .expect("the toolchain unit");
    assert_eq!(
        (
            &toolchain["from"],
            &toolchain["to"],
            &toolchain["evidence"]["held_back"]
        ),
        (
            &json!("1.62.1"),
            &json!("1.100.0"),
            &json!(["itertools 0.15.0", "itoa 1.0.19"])
        )
    );
}

#[test]
fn run_of_check_after_a_requirement_edited_by_hand_exits_three() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files()
        .with("/work/ws/plan.json", PLAN)
        .with(
            "/work/ws/Cargo.lock",
            &moved(LOCK, "itoa", "0.4.7", "0.4.8"),
        )
        .with(
            "/work/ws/app/Cargo.toml",
            &include_str!("../fixtures/workspace/files/app/Cargo.toml.txt")
                .replace("smallvec = \"1.6.0\"", "smallvec = \"1.7\""),
        );

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:itoa"));

    // Assert
    assert_eq!(ran.code, 3);
    let document = parsed(&ran.output);
    assert_eq!(document["diagnostics"][0]["code"], "plan-stale");
    assert_eq!(document["diagnostics"][0]["where"], "app/Cargo.toml:12");
}

#[test]
fn run_of_check_in_a_table_says_how_the_unit_stands() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN).with(
        "/work/ws/Cargo.lock",
        &moved(LOCK, "itoa", "0.4.7", "0.4.8"),
    );

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&args(&[
        "check",
        "compatible:itoa",
        "--plan",
        "plan.json",
    ]));

    // Assert
    assert_eq!(ran.code, 0);
    assert_eq!(
        ran.output,
        "check compatible:itoa: as planned\napplied: compatible:itoa\nto go: compatible:itertools, breaking:itertools, breaking:itoa, compatible:memchr, toolchain:rust, compatible:smallvec\n"
    );
}

#[test]
fn run_of_check_of_a_plan_of_another_schema_exits_two() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with(
        "/work/ws/plan.json",
        &PLAN.replacen("\"schema\": 1", "\"schema\": 2", 1),
    );

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:itoa"));

    // Assert
    assert_eq!(ran.code, 2);
    assert_eq!(
        parsed(&ran.output)["diagnostics"][0]["code"],
        "plan-schema-unknown"
    );
}

#[test]
fn run_of_check_on_a_unit_applied_as_planned_exits_zero_listing_what_is_applied() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN).with(
        "/work/ws/Cargo.lock",
        &moved(LOCK, "itoa", "0.4.7", "0.4.8"),
    );

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:itoa"));

    // Assert
    assert_eq!(ran.code, 0);
    let document = parsed(&ran.output);
    assert_eq!(document["command"], "check");
    assert_eq!(document["unit"], "compatible:itoa");
    assert_eq!(document["as_planned"], true);
    assert_eq!(document["applied"], json!(["compatible:itoa"]));
    assert_eq!(
        document["to_go"],
        json!([
            "compatible:itertools",
            "breaking:itertools",
            "breaking:itoa",
            "compatible:memchr",
            "toolchain:rust",
            "compatible:smallvec"
        ])
    );
}

#[test]
fn run_of_check_on_a_unit_not_yet_applied_exits_one_naming_its_move() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:itoa"));

    // Assert
    assert_eq!(ran.code, 1);
    assert_eq!(
        parsed(&ran.output)["differences"][0]["what"],
        "`itoa` is still 0.4.7, where the unit moves it to 0.4.8"
    );
}

#[test]
fn run_of_check_on_a_unit_that_moved_more_exits_one_with_each_difference() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN).with(
        "/work/ws/Cargo.lock",
        &moved(
            &moved(LOCK, "itoa", "0.4.7", "0.4.8"),
            "memchr",
            "2.5.0",
            "2.6.0",
        ),
    );

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:itoa"));

    // Assert
    assert_eq!(ran.code, 1);
    assert_eq!(
        parsed(&ran.output)["differences"],
        json!([{
            "where": "Cargo.lock",
            "what": "`memchr` moved 2.5.0 -> 2.6.0, a move no unit of the plan makes",
            "do": "carry out the unit's `revert`, `git restore -- Cargo.lock`: compatible:itoa becomes ask, with these differences attached"
        }])
    );
}

#[test]
fn run_of_check_on_a_unit_the_plan_does_not_hold_exits_two() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&check("compatible:serde"));

    // Assert
    assert_eq!(ran.code, 2);
    assert_eq!(
        parsed(&ran.output)["diagnostics"][0]["code"],
        "unit-unknown"
    );
}

#[test]
fn run_of_guide_prints_the_guide_reading_nothing_and_exits_zero() {
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
    assert_eq!(ran.code, 0);
    assert!(
        ran.output.starts_with(&format!(
            "ripe4rust {}: the guide\n\nThe round\n1. Run `cargo ripe4rust plan --format json > plan.json`.\n",
            env!("CARGO_PKG_VERSION")
        )),
        "{}",
        ran.output
    );
    assert_eq!(runner.commands(), Vec::<String>::new());
}

#[test]
fn run_of_hold_in_a_table_prints_the_pin_and_what_to_do() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&args(&[
        "hold",
        "breaking:itoa",
        "--plan",
        "plan.json",
        "--because",
        "the person: wait",
        "--review",
        "2027-02-01",
    ]));

    // Assert
    assert_eq!(ran.code, 0);
    assert_eq!(
        ran.output,
        "hold breaking:itoa: under [workspace.metadata.ripe4rust.pins] at Cargo.toml:19\nitoa = { reason = \"the person: wait\", review = \"2027-02-01\" }\nDo: put the pin under `[workspace.metadata.ripe4rust.pins]` at Cargo.toml:19, and commit it\n"
    );
}

#[test]
fn run_of_hold_on_a_unit_prints_its_pin_under_the_pins_table() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock))
        .run(&hold("compatible:itoa", "stage 2 failed"));

    // Assert
    assert_eq!(ran.code, 0);
    assert_eq!(
        parsed(&ran.output),
        json!({
            "schema": 1,
            "command": "hold",
            "unit": "compatible:itoa",
            "section": "[workspace.metadata.ripe4rust.pins]",
            "where": "Cargo.toml:19",
            "pins": [{
                "crate": "itoa",
                "line": "itoa = { reason = \"stage 2 failed\", review = \"2026-12-10\", hold = \"all\" }",
                "replaces": null
            }],
            "do": "put the pin under `[workspace.metadata.ripe4rust.pins]` at Cargo.toml:19, and commit it",
            "diagnostics": []
        })
    );
}

#[test]
fn run_of_hold_on_an_urgent_unit_exits_two_saying_it_becomes_an_urgent_ask() {
    // Arrange
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);

    // Act
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock))
        .run(&hold("compatible:smallvec", "stage 2 failed"));

    // Assert
    assert_eq!(ran.code, 2);
    assert_eq!(parsed(&ran.output)["diagnostics"][0]["code"], "unit-urgent");
}

#[test]
fn run_of_hold_prints_a_pin_the_policy_reads_back_as_written() {
    // Arrange
    let because = "stage 2: \"crap4rust\" over 15 in C:\\ws\tand a tab";
    let (runner, pause, clock) = (layout(), CountingPause::new(), clock());
    let files = files().with("/work/ws/plan.json", PLAN);
    let ran = Ripe4rust::new(seams(&runner, &files, &pause, &clock))
        .run(&hold("compatible:itoa", because));
    let line = parsed(&ran.output)["pins"][0]["line"]
        .as_str()
        .map(String::from)
        .expect("a pin");
    let manifest = format!(
        "{}{line}\n",
        include_str!("../fixtures/workspace/files/Cargo.toml.txt")
    );

    // Act
    let policy = PolicyReader.read(&PolicySection.tree(&manifest).expect("a policy"));

    // Assert
    assert_eq!(policy.diagnostics, Vec::new());
    let pin = policy
        .value
        .pins
        .iter()
        .find(|pin| pin.name == "itoa")
        .expect("the pin");
    assert_eq!(
        (
            pin.reason.as_str(),
            pin.hold,
            pin.review.map(|review| review.to_string())
        ),
        (because, Hold::All, Some(String::from("2026-12-10")))
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
fn run_of_version_prints_the_binary_and_its_release_reading_nothing_and_exits_zero() {
    // Arrange
    let (runner, files, pause, clock) = (
        ReplayRunner::new(),
        MemoryFiles::new(),
        CountingPause::new(),
        clock(),
    );
    let ripe4rust = Ripe4rust::new(seams(&runner, &files, &pause, &clock));

    // Act
    let ran = [
        ripe4rust.run(&args(&["ripe4rust", "--version"])),
        ripe4rust.run(&args(&["-V"])),
    ];

    // Assert
    let printed = format!("cargo-ripe4rust {}\n", env!("CARGO_PKG_VERSION"));
    assert_eq!(
        ran.map(|ran| (ran.output, ran.code)),
        [(printed.clone(), 0), (printed, 0)]
    );
    assert_eq!(runner.commands(), Vec::<String>::new());
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
fn run_refreshed_after_an_online_run_fetches_every_answer_again_over_the_day_s_copy() {
    // Arrange
    let (online, files, pause, clock) = (recorded(false), files(), CountingPause::new(), clock());
    let first =
        Ripe4rust::new(seams(&online, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));
    let cached = "/cache/2026-11-26/toolchain/manifests.txt";
    let kept = files.contents(cached);
    let files = files.with(cached, "an answer read in the morning");
    let refreshed = recorded(false);
    let mut words = plan_json("2026-11-26", false);
    words.push(String::from("--refresh"));

    // Act
    let second = Ripe4rust::new(seams(&refreshed, &files, &pause, &clock)).run(&words);

    // Assert
    let fetched = |runner: &ReplayRunner| {
        runner
            .commands()
            .into_iter()
            .filter(|command| command.starts_with("curl"))
            .collect::<Vec<String>>()
    };
    assert_eq!((second.code, &second.output), (0, &first.output));
    assert_eq!(fetched(&refreshed), fetched(&online));
    assert_eq!(files.contents(cached), kept);
}

#[test]
fn run_with_the_rust_distribution_out_of_reach_prints_no_plan_and_exits_two() {
    // Arrange
    let runner = recorded(false).reply(
        &curl("https://static.rust-lang.org/manifests.txt"),
        failed(
            6,
            include_str!("../fixtures/toolchain/manifests-unreachable.txt"),
        ),
    );
    let mut runner = runner;
    let unreachable = runner.replies.pop().expect("the reply");
    runner.replies.insert(0, unreachable);
    let (files, pause, clock) = (files(), CountingPause::new(), clock());

    // Act
    let ran =
        Ripe4rust::new(seams(&runner, &files, &pause, &clock)).run(&plan_json("2026-11-26", false));

    // Assert
    let document = parsed(&ran.output);
    assert_eq!(
        (ran.code, &document["diagnostics"][0]["code"]),
        (2, &json!("toolchain-unreachable"))
    );
    assert!(document.get("units").is_none(), "{}", ran.output);
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

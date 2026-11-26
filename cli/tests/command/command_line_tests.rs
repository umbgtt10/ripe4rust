// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::command::command_line::CommandLine;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::request::Request;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::values::date::Date;

const CHECK_USAGE: &str = "run `cargo ripe4rust check <unit> --plan <file> [--format table|json]`";
const USAGE: &str =
    "run `cargo ripe4rust [plan] [--format table|json] [--date YYYY-MM-DD] [--offline]`";

fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| String::from(*word)).collect()
}

fn plan(format: Format, date: Option<&str>, offline: bool) -> Request {
    Request {
        command: String::from("plan"),
        format,
        date: date.map(|date| Date::parse(date).expect("a date")),
        offline,
        unit: None,
        plan: None,
        because: None,
        review: None,
    }
}

fn refused(words: &[&str]) -> (DiagnosticCode, String, String) {
    let refusal = CommandLine.read(&args(words)).expect_err("a refusal");
    (
        refusal.diagnostic.code,
        refusal.diagnostic.what,
        refusal.diagnostic.action,
    )
}

#[test]
fn read_of_a_command_not_built_yet_is_unavailable() {
    // Arrange & Act
    let refusals = ["hold", "guide"].map(|command| {
        CommandLine.read(&args(&[
            "ripe4rust",
            command,
            "compatible:itoa",
            "--plan",
            "plan.json",
        ]))
    });

    // Assert
    for (refusal, command) in refusals.iter().zip(["hold", "guide"]) {
        let refusal = refusal.as_ref().expect_err("a refusal");
        assert_eq!(refusal.command, command);
        assert_eq!(refusal.diagnostic.code, DiagnosticCode::CommandUnavailable);
        assert_eq!(
            refusal.diagnostic.what,
            format!("`{command}` is not in this release of ripe4rust yet")
        );
        assert_eq!(
            refusal.diagnostic.action,
            "use `cargo ripe4rust plan` or `cargo ripe4rust check`, the commands this release runs; `hold` and `guide` come with a later release"
        );
    }
}

#[test]
fn read_of_a_date_that_is_no_date_is_usage_invalid() {
    // Arrange & Act
    let refusal = CommandLine
        .read(&args(&["plan", "--date", "2026-13-01"]))
        .expect_err("a refusal");

    // Assert
    assert_eq!(
        (
            refusal.diagnostic.code,
            refusal.diagnostic.what,
            refusal.diagnostic.action
        ),
        (
            DiagnosticCode::UsageInvalid,
            String::from("`--date` takes a date, YYYY-MM-DD, not `2026-13-01`"),
            String::from(USAGE)
        )
    );
}

#[test]
fn read_of_a_format_it_does_not_know_is_usage_invalid() {
    // Arrange & Act
    let refusal = CommandLine
        .read(&args(&["--format", "yaml"]))
        .expect_err("a refusal");

    // Assert
    assert_eq!(
        (refusal.diagnostic.code, refusal.diagnostic.what),
        (
            DiagnosticCode::UsageInvalid,
            String::from("`--format` takes `table` or `json`, not `yaml`")
        )
    );
}

#[test]
fn read_of_an_option_given_no_value_is_usage_invalid() {
    // Arrange & Act
    let refusal = CommandLine.read(&args(&["--date"])).expect_err("a refusal");

    // Assert
    assert_eq!(
        refusal.diagnostic.what,
        "`--date` takes a date, YYYY-MM-DD, and was given none"
    );
}

#[test]
fn read_of_an_option_it_does_not_know_is_usage_invalid() {
    // Arrange & Act
    let refusal = CommandLine
        .read(&args(&["plan", "--fromat", "json"]))
        .expect_err("a refusal");

    // Assert
    assert_eq!(
        (refusal.command, refusal.format, refusal.diagnostic.what),
        (
            String::from("plan"),
            Format::Table,
            String::from("`--fromat` is not an option of `plan`")
        )
    );
}

#[test]
fn read_of_an_unknown_command_is_usage_invalid_and_answers_in_the_format_asked() {
    // Arrange & Act
    let refusal = CommandLine
        .read(&args(&["ripe4rust", "frobnicate", "--format=json"]))
        .expect_err("a refusal");

    // Assert
    assert_eq!(
        (
            refusal.command,
            refusal.format,
            refusal.diagnostic.code,
            refusal.diagnostic.what
        ),
        (
            String::from("frobnicate"),
            Format::Json,
            DiagnosticCode::UsageInvalid,
            String::from("`frobnicate` is not a command of ripe4rust")
        )
    );
}

#[test]
fn read_of_check_given_an_option_of_plan_is_usage_invalid() {
    // Arrange & Act
    let refusal = refused(&[
        "check",
        "compatible:itoa",
        "--plan",
        "plan.json",
        "--offline",
    ]);

    // Assert
    assert_eq!(
        refusal,
        (
            DiagnosticCode::UsageInvalid,
            String::from("`--offline` is not an option of `check`"),
            String::from(CHECK_USAGE)
        )
    );
}

#[test]
fn read_of_check_given_two_units_is_usage_invalid() {
    // Arrange & Act
    let refusal = refused(&[
        "check",
        "compatible:itoa",
        "breaking:itoa",
        "--plan",
        "plan.json",
    ]);

    // Assert
    assert_eq!(
        refusal.1,
        "`breaking:itoa` is a second unit, and `check` takes one"
    );
}

#[test]
fn read_of_check_takes_its_unit_and_its_plan() {
    // Arrange & Act
    let request = CommandLine.read(&args(&[
        "ripe4rust",
        "check",
        "compatible:itoa/itoa",
        "--plan=plans/today.json",
        "--format",
        "json",
    ]));

    // Assert
    assert_eq!(
        request,
        Ok(Request {
            command: String::from("check"),
            format: Format::Json,
            date: None,
            offline: false,
            unit: Some(String::from("compatible:itoa/itoa")),
            plan: Some(String::from("plans/today.json")),
            because: None,
            review: None,
        })
    );
}

#[test]
fn read_of_check_without_a_plan_is_usage_invalid() {
    // Arrange & Act
    let refusal = refused(&["check", "compatible:itoa"]);

    // Assert
    assert_eq!(
        refusal,
        (
            DiagnosticCode::UsageInvalid,
            String::from("`check` takes `--plan <file>`, and was given none"),
            String::from(CHECK_USAGE)
        )
    );
}

#[test]
fn read_of_check_without_a_unit_is_usage_invalid() {
    // Arrange & Act
    let refusal = refused(&["check", "--plan", "plan.json"]);

    // Assert
    assert_eq!(
        refusal.1,
        "`check` takes the id of a unit, and was given none"
    );
}

#[test]
fn read_of_every_option_takes_each() {
    // Arrange & Act
    let request = CommandLine.read(&args(&[
        "ripe4rust",
        "plan",
        "--format=json",
        "--date",
        "2026-10-30",
        "--offline",
    ]));

    // Assert
    assert_eq!(request, Ok(plan(Format::Json, Some("2026-10-30"), true)));
}

#[test]
fn read_of_nothing_is_a_plan_in_a_table_for_today() {
    // Arrange & Act
    let requests = [
        CommandLine.read(&[]),
        CommandLine.read(&args(&["ripe4rust"])),
    ];

    // Assert
    assert_eq!(
        requests,
        [
            Ok(plan(Format::Table, None, false)),
            Ok(plan(Format::Table, None, false))
        ]
    );
}

#[test]
fn read_of_options_without_the_command_is_a_plan() {
    // Arrange & Act
    let request = CommandLine.read(&args(&[
        "--offline",
        "--date=2026-11-26",
        "--format",
        "table",
    ]));

    // Assert
    assert_eq!(request, Ok(plan(Format::Table, Some("2026-11-26"), true)));
}

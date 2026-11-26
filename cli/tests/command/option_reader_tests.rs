// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::command::command_spec::CommandSpec;
use cargo_ripe4rust::command::format::Format;
use cargo_ripe4rust::command::option_reader::OptionReader;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::values::date::Date;

const USAGE: &str = "run `cargo ripe4rust check <unit> --plan <file>`";

fn reader() -> OptionReader {
    OptionReader::new(CommandSpec {
        name: "check",
        usage: USAGE,
        options: &["--format", "--plan"],
        unit: true,
        required: &["--plan"],
    })
}

#[test]
fn date_of_a_date_is_it_and_of_none_or_another_text_usage_invalid() {
    // Arrange
    let reader = reader();

    // Act
    let read = [
        reader.date("--review", Some("2027-01-31")),
        reader.date("--review", None),
        reader.date("--review", Some("soon")),
    ];

    // Assert
    assert_eq!(read[0], Ok(Date::parse("2027-01-31").expect("a date")));
    assert_eq!(
        read[1]
            .as_ref()
            .map_err(|diagnostic| diagnostic.what.as_str()),
        Err("`--review` takes a date, YYYY-MM-DD, and was given none")
    );
    assert_eq!(
        read[2]
            .as_ref()
            .map_err(|diagnostic| diagnostic.what.as_str()),
        Err("`--review` takes a date, YYYY-MM-DD, not `soon`")
    );
}

#[test]
fn format_of_each_format_is_it_and_of_anything_else_usage_invalid() {
    // Arrange
    let reader = reader();

    // Act
    let read = [
        reader.format(Some("json")),
        reader.format(Some("table")),
        reader.format(Some("yaml")),
    ]
    .map(|read| read.map_err(|diagnostic| diagnostic.code));

    // Assert
    assert_eq!(
        read,
        [
            Ok(Format::Json),
            Ok(Format::Table),
            Err(DiagnosticCode::UsageInvalid)
        ]
    );
}

#[test]
fn read_of_a_unit_and_a_plan_is_the_request() {
    // Arrange
    let reader = reader();

    // Act
    let request = reader
        .read(&["rest", "--plan", "plan.json"], Format::Table)
        .expect("a request");

    // Assert
    assert_eq!(
        (request.command.as_str(), request.unit, request.plan),
        (
            "check",
            Some(String::from("rest")),
            Some(String::from("plan.json"))
        )
    );
}

#[test]
fn text_of_nothing_or_blanks_is_usage_invalid_and_of_words_the_words() {
    // Arrange
    let reader = reader();

    // Act
    let read = [
        reader.text("--because", "the reason", Some("stage 2 failed")),
        reader.text("--because", "the reason", Some("  ")),
        reader.text("--because", "the reason", None),
    ];

    // Assert
    assert_eq!(read[0], Ok(String::from("stage 2 failed")));
    for refused in &read[1..] {
        assert_eq!(
            refused
                .as_ref()
                .map_err(|diagnostic| diagnostic.what.as_str()),
            Err("`--because` takes the reason, and was given none")
        );
    }
}

#[test]
fn usage_is_usage_invalid_with_the_command_s_usage() {
    // Arrange & Act
    let diagnostic = reader().usage(String::from("`--date` is not an option of `check`"));

    // Assert
    assert_eq!(
        (diagnostic.code, diagnostic.action.as_str()),
        (DiagnosticCode::UsageInvalid, USAGE)
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::plan::plan_reader::PlanReader;
use ripe4rust_core::inputs::git::git_entry::GitEntry;
use ripe4rust_core::inputs::lock_edge::LockEdge;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::version::Version;

const PLAN: &str = include_str!("../../../fixtures/workspace/plan.json");

fn unreadable(detail: &str) -> PortFailure {
    PortFailure::PlanUnreadable {
        file: String::from("plan.json"),
        detail: String::from(detail),
    }
}

#[test]
fn plan_of_a_document_of_another_command_is_unreadable() {
    // Arrange & Act
    let plan = PlanReader.plan("plan.json", "{ \"schema\": 1, \"command\": \"check\" }");

    // Assert
    assert_eq!(
        plan,
        Err(unreadable("it is the document of `check`, not of `plan`"))
    );
}

#[test]
fn plan_of_a_document_with_no_schema_is_unreadable() {
    // Arrange & Act
    let plan = PlanReader.plan("plan.json", "{ \"command\": \"plan\" }");

    // Assert
    assert_eq!(
        plan,
        Err(unreadable(
            "it carries no `schema`, so it is no document ripe4rust wrote"
        ))
    );
}

#[test]
fn plan_of_a_lock_entry_that_does_not_read_is_unreadable() {
    // Arrange
    let text = PLAN.replacen("\"either 1.5.0\"", "\"either\"", 1);

    // Act
    let plan = PlanReader.plan("plan.json", &text);

    // Assert
    assert_eq!(
        plan,
        Err(unreadable(
            "`either` in the lock is not a crate and its version"
        ))
    );
}

#[test]
fn plan_of_a_plan_holding_git_sources_reads_each_crate_at_its_rev() {
    // Arrange
    let text = PLAN.replace(
        "\"git\": [],",
        "\"git\": [\"gitcore https://github.com/example/upstream?branch=main#9625c3d2\"],",
    );

    // Act
    let plan = PlanReader.plan("plan.json", &text).expect("a plan");

    // Assert
    assert_eq!(
        plan.git,
        vec![GitEntry {
            name: String::from("gitcore"),
            source: String::from("https://github.com/example/upstream?branch=main"),
            rev: String::from("9625c3d2"),
        }]
    );
}

#[test]
fn plan_of_a_plan_naming_the_crates_its_units_share_reads_them() {
    // Arrange
    let text = PLAN.replace(
        "\"shared\": [],",
        "\"shared\": [\"getrandom\", \"wasip2\"],",
    );

    // Act
    let plan = PlanReader.plan("plan.json", &text).expect("a plan");

    // Assert
    assert_eq!(
        plan.shared,
        vec![String::from("getrandom"), String::from("wasip2")]
    );
}

#[test]
fn plan_of_a_plan_recording_the_lock_s_edges_reads_each() {
    // Arrange
    let text = PLAN.replace(
        "\"edges\": [],",
        "\"edges\": [\"itertools 0.10.0 -> either 1.5.0\"],",
    );

    // Act
    let plan = PlanReader.plan("plan.json", &text).expect("a plan");

    // Assert
    assert_eq!(
        plan.edges,
        Some(vec![LockEdge {
            dependent: String::from("itertools"),
            dependent_version: Version::parse("0.10.0").expect("a version"),
            dependency: String::from("either"),
            dependency_version: Version::parse("1.5.0").expect("a version"),
        }])
    );
}

#[test]
fn plan_of_a_plan_that_failed_is_unreadable_saying_no_plan_was_made() {
    // Arrange & Act
    let plan = PlanReader.plan(
        "plan.json",
        "{ \"schema\": 1, \"command\": \"plan\", \"diagnostics\": [] }",
    );

    // Assert
    assert_eq!(
        plan,
        Err(unreadable(
            "it holds no units: the run that wrote it made no plan"
        ))
    );
}

#[test]
fn plan_of_a_plan_written_before_the_lock_s_edges_reads_none_recorded() {
    // Arrange
    let text = PLAN.replace("    \"edges\": [],\n", "");

    // Act
    let plan = PlanReader.plan("plan.json", &text).expect("a plan");

    // Assert
    assert_eq!(plan.edges, None);
}

#[test]
fn plan_of_a_plan_written_before_the_units_shared_crates_reads_none() {
    // Arrange
    let text = PLAN.replace("  \"shared\": [],\n", "");

    // Act
    let plan = PlanReader.plan("plan.json", &text).expect("a plan");

    // Assert
    assert_eq!(plan.shared, Vec::<String>::new());
}

#[test]
fn plan_of_an_edge_that_does_not_read_is_unreadable() {
    // Arrange
    let text = PLAN.replace(
        "\"edges\": [],",
        "\"edges\": [\"itertools 0.10.0 => either 1.5.0\"],",
    );

    // Act
    let plan = PlanReader.plan("plan.json", &text);

    // Assert
    assert_eq!(
        plan,
        Err(unreadable(
            "`itertools 0.10.0 => either 1.5.0` in the lock's edges is not a crate at its version depending on another at its own"
        ))
    );
}

#[test]
fn plan_of_another_schema_is_schema_unknown() {
    // Arrange & Act
    let plan = PlanReader.plan("plan.json", "{ \"schema\": 2, \"command\": \"plan\" }");

    // Assert
    assert_eq!(
        plan,
        Err(PortFailure::PlanSchemaUnknown {
            file: String::from("plan.json"),
            schema: String::from("2"),
        })
    );
}

#[test]
fn plan_of_text_that_is_not_json_is_unreadable() {
    // Arrange & Act
    let plan = PlanReader.plan("plan.json", "date 2026-11-26, registry read 2026-11-26");

    // Assert
    assert!(
        matches!(&plan, Err(PortFailure::PlanUnreadable { file, .. }) if file == "plan.json"),
        "{plan:?}"
    );
}

#[test]
fn plan_of_the_recorded_plan_is_its_date_its_workspace_and_its_units() {
    // Arrange & Act
    let plan = PlanReader.plan("plan.json", PLAN).expect("a plan");

    // Assert
    assert_eq!(plan.date, Date::parse("2026-11-26").expect("a date"));
    assert_eq!(plan.fingerprint, "af9871a4ebb0c8b9");
    assert_eq!(
        plan.lock[0],
        LockEntry {
            name: String::from("either"),
            version: Version::parse("1.5.0").expect("a version"),
        }
    );
    assert_eq!(plan.lock.len(), 5);
    assert_eq!(plan.edges, Some(Vec::new()));
    assert_eq!(
        plan.lines[1],
        WrittenLine {
            at: Location {
                file: String::from("Cargo.toml"),
                line: 12,
            },
            key: String::from("itertools"),
            value: String::from("0.10"),
        }
    );
    assert_eq!(
        plan.units
            .iter()
            .map(|unit| unit.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "compatible:itertools",
            "breaking:itertools",
            "compatible:itoa",
            "breaking:itoa",
            "compatible:memchr",
            "toolchain:rust",
            "compatible:smallvec",
        ]
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::compatible;
use crate::support::plans::plan;
use crate::support::reports::as_planned;
use cargo_ripe4rust::command::document::Document;
use cargo_ripe4rust::command::format::Format;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;

fn failure() -> PlanFailure {
    PlanFailure {
        diagnostics: vec![DiagnosticCode::CargoDenyMissing.diagnose(
            None,
            String::from("cargo-deny is not installed"),
            String::from("install it"),
        )],
    }
}

#[test]
fn check_in_a_table_opens_with_the_unit_and_how_it_stands() {
    // Arrange & Act
    let text = Document::new(Format::Table, "check").check(&as_planned());

    // Assert
    assert!(
        text.starts_with("check breaking:gadget: as planned\n"),
        "{text}"
    );
}

#[test]
fn check_in_json_is_the_check_document() {
    // Arrange & Act
    let text = Document::new(Format::Json, "check").check(&as_planned());

    // Assert
    assert!(
        text.starts_with(
            "{\n  \"schema\": 1,\n  \"command\": \"check\",\n  \"unit\": \"breaking:gadget\""
        ),
        "{text}"
    );
}

#[test]
fn failure_in_a_table_is_its_diagnostics_in_text() {
    // Arrange & Act
    let text = Document::new(Format::Table, "plan").failure(&failure());

    // Assert
    assert_eq!(
        text,
        "error[cargo-deny-missing] cargo-deny is not installed. Do: install it\n"
    );
}

#[test]
fn failure_in_json_is_the_failure_document() {
    // Arrange & Act
    let text = Document::new(Format::Json, "check").failure(&failure());

    // Assert
    assert!(
        text.starts_with("{\n  \"schema\": 1,\n  \"command\": \"check\",\n  \"diagnostics\": ["),
        "{text}"
    );
}

#[test]
fn plan_in_a_table_opens_with_its_date() {
    // Arrange & Act
    let text = Document::new(Format::Table, "plan").plan(&plan(vec![compatible()]));

    // Assert
    assert!(
        text.starts_with("date 2026-10-30, registry read 2026-10-30, workspace 9f2c41d0\n"),
        "{text}"
    );
}

#[test]
fn plan_in_json_is_the_plan_document() {
    // Arrange & Act
    let text = Document::new(Format::Json, "plan").plan(&plan(vec![compatible()]));

    // Assert
    assert!(
        text.starts_with(
            "{\n  \"schema\": 1,\n  \"command\": \"plan\",\n  \"date\": \"2026-10-30\""
        ),
        "{text}"
    );
}

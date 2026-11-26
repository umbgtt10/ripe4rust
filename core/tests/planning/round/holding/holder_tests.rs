// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_files::FakeFiles;
use crate::support::fakes::fake_plan::FakePlan;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use crate::support::policy_trees::texts;
use crate::support::policy_trees::tree;
use crate::support::recorded::base;
use crate::support::recorded::changed;
use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::plan_with_git;
use crate::support::recorded::with_git;
use crate::support::releases::date;
use crate::support::surveyed::at;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;
use ripe4rust_core::planning::round::holding::holder::Holder;
use ripe4rust_core::planning::round::holding::pin_line::PinLine;

const SECTION: &str = "[workspace.metadata.ripe4rust.pins]";

fn codes(failure: &PlanFailure) -> Vec<DiagnosticCode> {
    failure
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn hold(recorded: RecordedPlan, files: &FakeFiles, unit: &str) -> Result<HoldReport, PlanFailure> {
    let source = FakePlan { plan: Ok(recorded) };
    Holder::new(&source, files).hold(unit, "2.7.6 broke stage 2", None)
}

fn pinned() -> FakeFiles {
    let mut files = base();
    files.policy = tree(vec![
        entry("gates", texts(&["just stage1"]), 31),
        entry(
            "pins",
            PolicyValue::Table(vec![entry(
                "gizmo",
                PolicyValue::Table(vec![
                    entry("reason", text("3 is a migration"), 41),
                    entry("review", text("2027-03-01"), 41),
                ]),
                41,
            )]),
            40,
        ),
    ]);
    files
}

#[test]
fn hold_after_a_breaking_unit_applied_leaves_its_tail_to_it() {
    // Arrange
    let files = changed(
        lock(&[
            ("gadget", "0.14.0"),
            ("gizmo", "2.7.5"),
            ("libc", "0.2.177"),
            ("tiny", "1.0.0"),
            ("widget", "1.4.0"),
        ]),
        lines_with(&[(22, "0.14.0")]),
    );

    // Act
    let report = hold(plan(), &files, "rest");

    // Assert
    assert_eq!(report.map(|report| report.pins.len()), Ok(1));
}

#[test]
fn hold_after_a_move_no_unit_makes_is_the_plan_stale() {
    // Arrange
    let files = changed(
        lock(&[
            ("gadget", "0.13.1"),
            ("gizmo", "2.7.5"),
            ("libc", "0.2.180"),
            ("widget", "1.4.0"),
        ]),
        lines(),
    );

    // Act
    let failure = hold(plan(), &files, "compatible:widget").expect_err("a stale plan");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::PlanStale]);
    assert_eq!(
        failure.diagnostics[0].what,
        "in `Cargo.lock`, `libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes"
    );
}

#[test]
fn hold_against_a_requirement_edited_by_hand_is_the_plan_stale() {
    // Arrange
    let files = changed(plan().lock, lines_with(&[(21, "1.5")]));

    // Act
    let failure = hold(plan(), &files, "compatible:widget").expect_err("a stale plan");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::PlanStale]);
}

#[test]
fn hold_into_a_pins_table_puts_the_pin_under_it_in_place_of_one_on_the_crate() {
    // Arrange
    let files = pinned();

    // Act
    let report = hold(plan(), &files, "compatible:widget/gizmo").expect("a pin");

    // Assert
    assert_eq!(report.at, Some(at("Cargo.toml", 40)));
    assert_eq!(
        report.pins,
        vec![PinLine {
            name: String::from("gizmo"),
            line: String::from(
                r#"gizmo = { reason = "3 is a migration; 2.7.6 broke stage 2", review = "2026-12-10", hold = "all" }"#
            ),
            replaces: Some(at("Cargo.toml", 41)),
        }]
    );
}

#[test]
fn hold_of_a_compatible_unit_pins_each_move_reviewed_hold_days_after_the_plan() {
    // Arrange
    let files = base();

    // Act
    let report = hold(plan(), &files, "compatible:widget");

    // Assert
    assert_eq!(
        report,
        Ok(HoldReport {
            unit: String::from("compatible:widget"),
            section: String::from(SECTION),
            at: None,
            pins: vec![
                PinLine {
                    name: String::from("widget"),
                    line: String::from(
                        r#"widget = { reason = "2.7.6 broke stage 2", review = "2026-12-10", hold = "all" }"#
                    ),
                    replaces: None,
                },
                PinLine {
                    name: String::from("gizmo"),
                    line: String::from(
                        r#"gizmo = { reason = "2.7.6 broke stage 2", review = "2026-12-10", hold = "all" }"#
                    ),
                    replaces: None,
                },
            ],
            action: String::from(
                "add `[workspace.metadata.ripe4rust.pins]` at the end of `Cargo.toml`, put the pins under it, and commit it"
            ),
            diagnostics: Vec::new(),
        })
    );
}

#[test]
fn hold_of_a_git_unit_pins_its_crate_whole() {
    // Arrange
    let files = with_git("9625c3d2");

    // Act
    let report = hold(plan_with_git(), &files, "git:gitcore").expect("a pin");

    // Assert
    assert_eq!(
        report
            .pins
            .iter()
            .map(|pin| pin.line.clone())
            .collect::<Vec<String>>(),
        vec![String::from(
            "gitcore = { reason = \"2.7.6 broke stage 2\", review = \"2026-12-10\", hold = \"all\" }"
        )]
    );
}

#[test]
fn hold_of_an_advisory_row_is_refused_as_an_urgent_ask() {
    // Arrange
    let files = base();

    // Act
    let failure = hold(plan(), &files, "advisory:gizmo@2.7.5").expect_err("a refusal");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::UnitUrgent]);
}

#[test]
fn hold_of_an_unknown_unit_is_refused() {
    // Arrange
    let files = base();

    // Act
    let failure = hold(plan(), &files, "compatible:serde").expect_err("a refusal");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::UnitUnknown]);
}

#[test]
fn hold_of_an_urgent_unit_is_refused_saying_it_becomes_an_urgent_ask() {
    // Arrange
    let mut recorded = plan();
    recorded.units[2].urgent = true;
    let mut files = base();
    files.failure = Some(PortFailure::CurlMissing);

    // Act
    let failure = hold(recorded, &files, "compatible:widget").expect_err("a refusal");

    // Assert
    let refusal = &failure.diagnostics[0];
    assert_eq!(
        (refusal.code, refusal.needs_person),
        (DiagnosticCode::UnitUrgent, true)
    );
    assert_eq!(
        refusal.what,
        "`compatible:widget` is urgent, and an urgent unit is never held: holding it would keep the advisory or the yanked release it clears"
    );
    assert_eq!(
        refusal.action,
        "do not hold it: it becomes an urgent ask -- carry out its `revert` if it is applied, and put its question to the person, with the failure attached, ahead of every other"
    );
}

#[test]
fn hold_with_a_review_given_takes_it() {
    // Arrange
    let (source, files) = (FakePlan { plan: Ok(plan()) }, base());

    // Act
    let report = Holder::new(&source, &files)
        .hold(
            "breaking:gadget",
            "the person: plan it for spring",
            Some(date("2027-03-31")),
        )
        .expect("a pin");

    // Assert
    assert_eq!(
        report.pins[0].line,
        r#"gadget = { reason = "the person: plan it for spring", review = "2027-03-31" }"#
    );
}

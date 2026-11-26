// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_files::FakeFiles;
use crate::support::fakes::fake_plan::FakePlan;
use crate::support::recorded::base;
use crate::support::recorded::changed;
use crate::support::recorded::entry_of;
use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::transitive;
use crate::support::surveyed::at;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::checker::Checker;
use ripe4rust_core::planning::round::checking::difference::Difference;

const UNIT_ONLY: &str = "carry out the unit's `revert`, `git restore -- Cargo.lock`: compatible:widget becomes ask, with these differences attached";

fn check(files: &FakeFiles, unit: &str) -> Result<CheckReport, PlanFailure> {
    let source = FakePlan { plan: Ok(plan()) };
    Checker::new(&source, files).check(unit)
}

fn codes(failure: &PlanFailure) -> Vec<DiagnosticCode> {
    failure
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn ids(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| String::from(*name)).collect()
}

fn widget_applied() -> Vec<LockEntry> {
    lock(&[
        ("gadget", "0.13.1"),
        ("gizmo", "2.7.6"),
        ("libc", "0.2.177"),
        ("widget", "1.4.1"),
    ])
}

#[test]
fn check_of_a_breaking_unit_applied_reports_the_moves_its_edit_made() {
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
    let report = check(&files, "breaking:gadget").expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.transitive, vec![transitive("tiny", "1.0.0")]);
    assert_eq!(report.applied, ids(&["breaking:gadget"]));
}

#[test]
fn check_of_a_compatible_unit_after_a_breaking_one_leaves_its_tail_to_it() {
    // Arrange
    let files = changed(
        lock(&[
            ("gadget", "0.14.0"),
            ("gizmo", "2.7.6"),
            ("libc", "0.2.177"),
            ("tiny", "1.0.0"),
            ("widget", "1.4.1"),
        ]),
        lines_with(&[(22, "0.14.0")]),
    );

    // Act
    let report = check(&files, "compatible:widget").expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.transitive, Vec::new());
    assert_eq!(
        report.applied,
        ids(&["breaking:gadget", "compatible:widget"])
    );
}

#[test]
fn check_of_a_move_no_unit_makes_is_not_as_planned_with_what_to_do() {
    // Arrange
    let mut moved = widget_applied();
    moved[2] = entry_of("libc", "0.2.180");
    let files = changed(moved, lines());

    // Act
    let report = check(&files, "compatible:widget").expect("a check");

    // Assert
    assert!(!report.as_planned);
    assert_eq!(
        report.differences,
        vec![Difference {
            at: String::from("Cargo.lock"),
            what: String::from("`libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes"),
            action: String::from(UNIT_ONLY),
        }]
    );
}

#[test]
fn check_of_a_requirement_edited_by_hand_is_the_plan_stale() {
    // Arrange
    let files = changed(widget_applied(), lines_with(&[(21, "1.5")]));

    // Act
    let failure = check(&files, "compatible:widget").expect_err("a stale plan");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::PlanStale]);
    assert_eq!(failure.diagnostics[0].at, Some(at("Cargo.toml", 21)));
}

#[test]
fn check_of_a_split_move_applied_is_as_planned_and_lists_its_other_move_to_go() {
    // Arrange
    let files = changed(
        lock(&[
            ("gadget", "0.13.1"),
            ("gizmo", "2.7.6"),
            ("libc", "0.2.177"),
            ("widget", "1.4.0"),
        ]),
        lines(),
    );

    // Act
    let report = check(&files, "compatible:widget/gizmo").expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.applied, ids(&["compatible:widget/gizmo"]));
    assert_eq!(
        report.to_go,
        ids(&[
            "breaking:gadget",
            "toolchain:rust",
            "compatible:widget/widget",
            "breaking:widget",
            "rest"
        ])
    );
}

#[test]
fn check_of_a_unit_applied_is_as_planned_listing_what_is_applied_and_what_is_to_go() {
    // Arrange
    let files = changed(widget_applied(), lines());

    // Act
    let report = check(&files, "compatible:widget");

    // Assert
    assert_eq!(
        report,
        Ok(CheckReport {
            unit: String::from("compatible:widget"),
            as_planned: true,
            transitive: Vec::new(),
            differences: Vec::new(),
            applied: ids(&["compatible:widget"]),
            to_go: ids(&[
                "breaking:gadget",
                "toolchain:rust",
                "breaking:widget",
                "rest"
            ]),
            diagnostics: Vec::new(),
        })
    );
}

#[test]
fn check_of_a_unit_not_applied_is_not_as_planned_naming_each_move() {
    // Arrange
    let files = base();

    // Act
    let report = check(&files, "compatible:widget").expect("a check");

    // Assert
    assert!(!report.as_planned);
    assert_eq!(
        report
            .differences
            .iter()
            .map(|difference| difference.what.as_str())
            .collect::<Vec<_>>(),
        vec![
            "`widget` is still 1.4.0, where the unit moves it to 1.4.1",
            "`gizmo` is still 2.7.5, where the unit moves it to 2.7.6",
        ]
    );
}

#[test]
fn check_of_an_advisory_row_is_refused_as_nothing_to_check() {
    // Arrange
    let files = base();

    // Act
    let failure = check(&files, "advisory:RUSTSEC-2026-0001").expect_err("a refusal");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::UnitUnknown]);
    assert_eq!(
        failure.diagnostics[0].what,
        "`advisory:RUSTSEC-2026-0001` is an advisory row: it applies nothing, so there is nothing to check"
    );
}

#[test]
fn check_of_an_unknown_unit_is_refused() {
    // Arrange
    let files = base();

    // Act
    let failure = check(&files, "compatible:serde").expect_err("a refusal");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::UnitUnknown]);
}

#[test]
fn check_of_an_unreadable_plan_reads_no_workspace() {
    // Arrange
    let mut files = base();
    files.failure = Some(PortFailure::Internal {
        doing: String::from("reading the policy"),
        detail: String::from("it should not be read"),
    });
    let source = FakePlan {
        plan: Err(PortFailure::PlanUnreadable {
            file: String::from("plan.json"),
            detail: String::from("it is not there"),
        }),
    };

    // Act
    let failure = Checker::new(&source, &files)
        .check("compatible:widget")
        .expect_err("a failure");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::PlanUnreadable]);
}

#[test]
fn check_of_the_toolchain_applied_is_its_channel_written() {
    // Arrange
    let files = changed(plan().lock, lines_with(&[(9, "1.100.0")]));

    // Act
    let report = check(&files, "toolchain:rust").expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.applied, ids(&["toolchain:rust"]));
}

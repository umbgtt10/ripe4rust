// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_files::FakeFiles;
use crate::support::fakes::fake_plan::FakePlan;
use crate::support::recorded::base;
use crate::support::recorded::changed;
use crate::support::recorded::edge;
use crate::support::recorded::entry_of;
use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::plan_with_git;
use crate::support::recorded::plan_with_shared;
use crate::support::recorded::shared_changed;
use crate::support::recorded::transitive;
use crate::support::recorded::with_git;
use crate::support::surveyed::at;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::checker::Checker;
use ripe4rust_core::planning::round::checking::difference::Difference;

const BOON_THEN_PROPTEST: [(&str, &str); 7] = [
    ("ahash", "0.8.14"),
    ("h2", "0.4.20"),
    ("libc", "0.2.177"),
    ("proptest", "1.12.0"),
    ("r-efi", "5.3.0"),
    ("r-efi", "6.0.0"),
    ("rand", "0.10.4"),
];
const SHARED: &str =
    "the units share the crate, and its moves depend on the order they are applied in";
const UNIT_ONLY: &str = "carry out the unit's `revert`, `git restore -- Cargo.lock`: compatible:widget becomes ask, with these differences attached";

fn check(files: &FakeFiles, unit: &str) -> Result<CheckReport, PlanFailure> {
    let source = FakePlan { plan: Ok(plan()) };
    Checker::new(&source, files).check(unit)
}

fn check_shared(entries: &[(&str, &str)], unit: &str) -> CheckReport {
    let source = FakePlan {
        plan: Ok(plan_with_shared()),
    };
    Checker::new(&source, &shared_changed(entries))
        .check(unit)
        .expect("a check")
}

fn codes(failure: &PlanFailure) -> Vec<DiagnosticCode> {
    failure
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

fn codes_of(report: &CheckReport) -> Vec<DiagnosticCode> {
    report
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
fn check_of_a_git_unit_applied_finds_its_source_at_the_rev_it_takes() {
    // Arrange
    let source = FakePlan {
        plan: Ok(plan_with_git()),
    };
    let files = with_git("ce129c85");

    // Act
    let report = Checker::new(&source, &files)
        .check("git:gitcore")
        .expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.applied, ids(&["git:gitcore"]));
}

#[test]
fn check_of_a_git_unit_not_applied_names_its_source_still_at_its_rev() {
    // Arrange
    let source = FakePlan {
        plan: Ok(plan_with_git()),
    };
    let files = FakeFiles {
        fingerprint: String::from("0000000000000003"),
        ..with_git("9625c3d2")
    };

    // Act
    let report = Checker::new(&source, &files)
        .check("git:gitcore")
        .expect("a check");

    // Assert
    assert_eq!(
        (report.as_planned, report.differences),
        (
            false,
            vec![Difference {
                at: String::from("Cargo.lock"),
                what: String::from(
                    "`https://github.com/example/upstream?branch=main` is still at 9625c3d2, where the unit moves it to ce129c85"
                ),
                action: String::from(
                    "carry out the unit's `revert`, `git restore -- Cargo.lock`: git:gitcore becomes ask, with these differences attached"
                ),
            }]
        )
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
fn check_of_a_plan_without_edges_is_as_planned_saying_once_that_they_cannot_be_compared() {
    // Arrange
    let source = FakePlan {
        plan: Ok(RecordedPlan {
            edges: None,
            ..plan()
        }),
    };
    let mut files = changed(widget_applied(), lines());
    files.edges = vec![edge(("gadget", "0.13.1"), ("widget", "1.4.1"))];

    // Act
    let report = Checker::new(&source, &files)
        .check("compatible:widget")
        .expect("a check");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(report.repointed, Vec::<String>::new());
    assert_eq!(codes_of(&report), vec![DiagnosticCode::PlanWithoutEdges]);
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
fn check_of_a_split_move_of_a_shared_crate_not_made_is_not_as_planned() {
    // Arrange
    let entries = [
        ("ahash", "0.8.14"),
        ("getrandom", "0.3.4"),
        ("h2", "0.4.20"),
        ("libc", "0.2.177"),
        ("proptest", "1.11.0"),
        ("r-efi", "5.3.0"),
        ("r-efi", "6.0.0"),
        ("rand", "0.9.4"),
        ("wasip2", "1.0.3"),
    ];

    // Act
    let report = check_shared(&entries, "compatible:boon/wasip2");

    // Assert
    assert_eq!(
        (
            report.as_planned,
            report
                .differences
                .iter()
                .map(|difference| difference.what.as_str())
                .collect::<Vec<&str>>()
        ),
        (
            false,
            vec!["`wasip2` is still 1.0.3, where the unit moves it to 1.0.4"]
        )
    );
}

#[test]
fn check_of_a_unit_after_a_git_source_moved_where_no_unit_takes_it_names_the_move() {
    // Arrange
    let source = FakePlan {
        plan: Ok(plan_with_git()),
    };
    let files = FakeFiles {
        lock: widget_applied(),
        ..with_git("deadbeef")
    };

    // Act
    let report = Checker::new(&source, &files)
        .check("compatible:widget")
        .expect("a check");

    // Assert
    assert_eq!(
        report
            .differences
            .iter()
            .map(|difference| difference.what.clone())
            .collect::<Vec<String>>(),
        vec![String::from(
            "`https://github.com/example/upstream?branch=main` moved 9625c3d2 -> deadbeef, a move no unit of the plan makes"
        )]
    );
}

#[test]
fn check_of_a_unit_after_another_that_moved_a_crate_they_share_is_as_planned_saying_nothing_of_it()
{
    // Arrange
    let entries = BOON_THEN_PROPTEST;

    // Act
    let report = check_shared(&entries, "compatible:proptest");

    // Assert
    assert_eq!(
        report,
        CheckReport {
            unit: String::from("compatible:proptest"),
            as_planned: true,
            transitive: Vec::new(),
            differences: Vec::new(),
            shared: Vec::new(),
            repointed: Vec::new(),
            applied: ids(&["compatible:boon", "compatible:proptest"]),
            to_go: ids(&["compatible:tonic", "rest"]),
            diagnostics: Vec::new(),
        }
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
            shared: Vec::new(),
            repointed: Vec::new(),
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
fn check_of_a_unit_beside_a_dependency_repointed_between_locked_versions_is_as_planned_naming_it() {
    // Arrange
    let tiny = [("tiny", "1.0.0"), ("tiny", "2.0.0")];
    let mut base = plan().lock;
    base.extend(lock(&tiny));
    let source = FakePlan {
        plan: Ok(RecordedPlan {
            lock: base,
            edges: Some(vec![edge(("gadget", "0.13.1"), ("tiny", "1.0.0"))]),
            ..plan()
        }),
    };
    let mut now = widget_applied();
    now.extend(lock(&tiny));
    let mut files = changed(now, lines());
    files.edges = vec![edge(("gadget", "0.13.1"), ("tiny", "2.0.0"))];

    // Act
    let report = Checker::new(&source, &files)
        .check("compatible:widget")
        .expect("a check");

    // Assert
    assert_eq!(
        (report.as_planned, report.differences, report.repointed),
        (
            true,
            Vec::new(),
            vec![String::from(
                "`gadget` 0.13.1's `tiny` 1.0.0 -> 2.0.0, both already in the lock"
            )]
        )
    );
    assert_eq!(report.diagnostics, Vec::new());
}

#[test]
fn check_of_a_unit_beside_shared_crates_that_moves_a_crate_no_unit_shares_is_not_as_planned() {
    // Arrange
    let mut entries = BOON_THEN_PROPTEST.to_vec();
    entries[1] = ("h2", "0.4.22");

    // Act
    let report = check_shared(&entries, "compatible:proptest");

    // Assert
    assert!(!report.as_planned);
    assert_eq!(
        report
            .differences
            .iter()
            .map(|difference| difference.what.as_str())
            .collect::<Vec<&str>>(),
        vec!["`h2` moved 0.4.20 -> 0.4.22, a move no unit of the plan makes"]
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
fn check_of_a_unit_still_reports_a_change_no_move_explains_of_a_shared_crate_it_moves() {
    // Arrange
    let mut entries = BOON_THEN_PROPTEST.to_vec();
    entries[1] = ("h2", "0.4.21");
    entries[5] = ("r-efi", "6.0.1");

    // Act
    let report = check_shared(&entries, "compatible:tonic");

    // Assert
    assert_eq!(
        (report.as_planned, report.shared),
        (
            true,
            vec![format!("`r-efi` 6.0.1 joined the lock: {SHARED}")]
        )
    );
}

#[test]
fn check_of_a_unit_whose_move_of_a_shared_crate_came_out_otherwise_is_as_planned_reporting_it() {
    // Arrange
    let mut entries = BOON_THEN_PROPTEST.to_vec();
    entries[1] = ("h2", "0.4.21");

    // Act
    let report = check_shared(&entries, "compatible:tonic");

    // Assert
    assert!(report.as_planned, "{report:?}");
    assert_eq!(
        report.shared,
        vec![format!(
            "`r-efi` 6.0.0 is still in the lock, where the unit removes it: {SHARED}"
        )]
    );
}

#[test]
fn check_of_a_unit_whose_own_crate_the_units_share_holds_it_to_its_move() {
    // Arrange
    let mut recorded = plan_with_shared();
    recorded.shared.push(String::from("proptest"));
    recorded.shared.sort();
    let source = FakePlan { plan: Ok(recorded) };
    let mut entries = BOON_THEN_PROPTEST.to_vec();
    entries[3] = ("proptest", "1.11.0");

    // Act
    let report = Checker::new(&source, &shared_changed(&entries))
        .check("compatible:proptest")
        .expect("a check");

    // Assert
    assert_eq!(
        (
            report.as_planned,
            report
                .differences
                .iter()
                .map(|difference| difference.what.as_str())
                .collect::<Vec<&str>>()
        ),
        (
            false,
            vec!["`proptest` is still 1.11.0, where the unit moves it to 1.12.0"]
        )
    );
}

#[test]
fn check_of_an_advisory_row_is_refused_as_nothing_to_check() {
    // Arrange
    let files = base();

    // Act
    let failure = check(&files, "advisory:gizmo@2.7.5").expect_err("a refusal");

    // Assert
    assert_eq!(codes(&failure), vec![DiagnosticCode::UnitUnknown]);
    assert_eq!(
        failure.diagnostics[0].what,
        "`advisory:gizmo@2.7.5` is an advisory row: it applies nothing, so there is nothing to check"
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
fn check_of_the_rest_not_applied_is_not_as_planned_naming_the_move_its_step_takes() {
    // Arrange
    let entries = [
        ("ahash", "0.8.14"),
        ("getrandom", "0.3.4"),
        ("h2", "0.4.20"),
        ("libc", "0.2.177"),
        ("proptest", "1.11.0"),
        ("r-efi", "5.3.0"),
        ("r-efi", "6.0.0"),
        ("rand", "0.9.4"),
        ("wasip2", "1.0.4"),
    ];

    // Act
    let report = check_shared(&entries, "rest");

    // Assert
    assert_eq!((report.as_planned, report.differences.len()), (false, 1));
    assert_eq!(
        report.differences[0].what,
        "`libc` is still 0.2.177, where the unit moves it to 0.2.178"
    );
}

#[test]
fn check_of_the_rest_whose_shared_moves_wait_on_units_not_applied_is_as_planned_reporting_them() {
    // Arrange
    let entries = [
        ("ahash", "0.8.14"),
        ("getrandom", "0.3.4"),
        ("h2", "0.4.20"),
        ("libc", "0.2.178"),
        ("proptest", "1.11.0"),
        ("r-efi", "5.3.0"),
        ("r-efi", "6.0.0"),
        ("rand", "0.9.4"),
        ("wasip2", "1.0.4"),
    ];

    // Act
    let report = check_shared(&entries, "rest");

    // Assert
    assert_eq!(
        (report.as_planned, report.differences, report.shared),
        (
            true,
            Vec::new(),
            vec![
                format!(
                    "`getrandom` 0.3.4 is still in the lock, where the unit removes it: {SHARED}"
                ),
                format!("`r-efi` 5.3.0 is still in the lock, where the unit removes it: {SHARED}"),
            ]
        )
    );
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

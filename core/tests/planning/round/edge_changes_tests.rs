// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::edge;
use crate::support::recorded::lock;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::diagnostics::severity::Severity;
use ripe4rust_core::inputs::lock_edge::LockEdge;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::planning::round::edge_changes::EdgeChanges;
use ripe4rust_core::planning::round::lock_changes::LockChanges;

const BOTH: [(&str, &str); 4] = [
    ("errno", "0.3.14"),
    ("rustix", "1.1.2"),
    ("windows-sys", "0.52.0"),
    ("windows-sys", "0.61.2"),
];

fn both() -> Vec<LockEntry> {
    lock(&BOTH)
}

fn changes(
    base: &[LockEntry],
    current: &[LockEntry],
    edges: (Option<&[LockEdge]>, &[LockEdge]),
) -> EdgeChanges {
    EdgeChanges::new(LockChanges::new(base, current), edges.0, edges.1)
}

fn on_windows_sys(dependent: (&str, &str), version: &str) -> LockEdge {
    edge(dependent, ("windows-sys", version))
}

#[test]
fn repointed_of_a_dependency_moved_between_versions_both_locks_hold_names_it() {
    // Arrange
    let recorded = [
        on_windows_sys(("errno", "0.3.14"), "0.61.2"),
        on_windows_sys(("rustix", "1.1.2"), "0.61.2"),
    ];
    let now = [
        on_windows_sys(("errno", "0.3.14"), "0.52.0"),
        on_windows_sys(("rustix", "1.1.2"), "0.61.2"),
    ];

    // Act
    let repointed = changes(&both(), &both(), (Some(&recorded), &now)).repointed();

    // Assert
    assert_eq!(
        repointed,
        vec![String::from(
            "`errno` 0.3.14's `windows-sys` 0.61.2 -> 0.52.0, both already in the lock"
        )]
    );
}

#[test]
fn repointed_of_a_dependency_moved_to_a_version_new_to_the_lock_is_none() {
    // Arrange
    let now_locked = lock(&[
        ("errno", "0.3.14"),
        ("rustix", "1.1.2"),
        ("windows-sys", "0.52.0"),
        ("windows-sys", "0.60.2"),
        ("windows-sys", "0.61.2"),
    ]);
    let recorded = [on_windows_sys(("errno", "0.3.14"), "0.61.2")];
    let now = [on_windows_sys(("errno", "0.3.14"), "0.60.2")];

    // Act
    let repointed = changes(&both(), &now_locked, (Some(&recorded), &now)).repointed();

    // Assert
    assert_eq!(repointed, Vec::<String>::new());
}

#[test]
fn repointed_of_a_dependency_whose_old_version_left_the_lock_is_none() {
    // Arrange
    let now_locked = lock(&[
        ("errno", "0.3.14"),
        ("rustix", "1.1.2"),
        ("windows-sys", "0.52.0"),
    ]);
    let recorded = [on_windows_sys(("errno", "0.3.14"), "0.61.2")];
    let now = [on_windows_sys(("errno", "0.3.14"), "0.52.0")];

    // Act
    let repointed = changes(&both(), &now_locked, (Some(&recorded), &now)).repointed();

    // Assert
    assert_eq!(repointed, Vec::<String>::new());
}

#[test]
fn repointed_of_a_dependent_at_another_version_now_is_none() {
    // Arrange
    let now_locked = lock(&[
        ("errno", "0.3.15"),
        ("rustix", "1.1.2"),
        ("windows-sys", "0.52.0"),
        ("windows-sys", "0.61.2"),
    ]);
    let recorded = [on_windows_sys(("errno", "0.3.14"), "0.61.2")];
    let now = [on_windows_sys(("errno", "0.3.15"), "0.52.0")];

    // Act
    let repointed = changes(&both(), &now_locked, (Some(&recorded), &now)).repointed();

    // Assert
    assert_eq!(repointed, Vec::<String>::new());
}

#[test]
fn repointed_of_a_plan_without_edges_is_none() {
    // Arrange
    let now = [on_windows_sys(("errno", "0.3.14"), "0.52.0")];

    // Act
    let repointed = changes(&both(), &both(), (None, &now)).repointed();

    // Assert
    assert_eq!(repointed, Vec::<String>::new());
}

#[test]
fn repointed_of_edges_as_the_plan_found_them_is_none() {
    // Arrange
    let recorded = [
        on_windows_sys(("errno", "0.3.14"), "0.61.2"),
        on_windows_sys(("rustix", "1.1.2"), "0.52.0"),
    ];

    // Act
    let repointed = changes(&both(), &both(), (Some(&recorded), &recorded)).repointed();

    // Assert
    assert_eq!(repointed, Vec::<String>::new());
}

#[test]
fn repointed_of_one_of_two_versions_a_dependent_takes_names_the_one_moved() {
    // Arrange
    let locked = lock(&[
        ("rand", "0.8.5"),
        ("rand_core", "0.5.1"),
        ("rand_core", "0.6.4"),
        ("rand_core", "0.9.3"),
    ]);
    let on_rand_core = |version: &str| edge(("rand", "0.8.5"), ("rand_core", version));
    let recorded = [on_rand_core("0.5.1"), on_rand_core("0.6.4")];
    let now = [on_rand_core("0.5.1"), on_rand_core("0.9.3")];

    // Act
    let repointed = changes(&locked, &locked, (Some(&recorded), &now)).repointed();

    // Assert
    assert_eq!(
        repointed,
        vec![String::from(
            "`rand` 0.8.5's `rand_core` 0.6.4 -> 0.9.3, both already in the lock"
        )]
    );
}

#[test]
fn unseen_of_a_plan_with_edges_is_none() {
    // Arrange & Act
    let unseen = changes(&both(), &both(), (Some(&[]), &[])).unseen();

    // Assert
    assert_eq!(unseen, None);
}

#[test]
fn unseen_of_a_plan_without_edges_says_once_that_they_cannot_be_compared() {
    // Arrange & Act
    let unseen = changes(&both(), &both(), (None, &[])).unseen();

    // Assert
    assert_eq!(
        unseen.map(|notice| (
            notice.code,
            notice.severity,
            notice.at,
            notice.what,
            notice.action
        )),
        Some((
            DiagnosticCode::PlanWithoutEdges,
            Severity::Notice,
            None,
            String::from(
                "the plan records no edges of the lock -- ripe4rust 0.1.2 and earlier wrote none --, so a dependency re-pointed between versions the lock already held cannot be seen"
            ),
            String::from(
                "nothing stops the check; to see re-points, plan again once this unit is committed or reverted -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan"
            ),
        ))
    );
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::SOURCE;
use crate::support::recorded::git_at;
use crate::support::recorded::lines;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use crate::support::recorded::plan_with_git;
use crate::support::recorded::transitive;
use crate::support::surveyed::at;
use crate::support::units::moved;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::round::current_state::CurrentState;
use ripe4rust_core::planning::round::sorting::Sorting;

fn ids(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| String::from(*name)).collect()
}

#[test]
fn git_of_a_plan_and_a_workspace_reads_each_source_at_its_rev_now() {
    // Arrange
    let plan = plan_with_git();
    let current = CurrentState {
        lock: plan.lock.clone(),
        lines: lines(),
        git: git_at("ce129c85"),
    };

    // Act
    let rev = Sorting::new(&plan, &current).git().rev(SOURCE);

    // Assert
    assert_eq!(rev, Some(String::from("ce129c85")));
}

#[test]
fn sort_of_a_breaking_unit_applied_names_it_and_leaves_its_tail_unexplained() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: lock(&[
                ("gadget", "0.14.0"),
                ("gizmo", "2.7.5"),
                ("libc", "0.2.177"),
                ("tiny", "1.0.0"),
                ("widget", "1.4.0"),
            ]),
            lines: lines_with(&[(22, "0.14.0")]),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(sorted.breaking, ids(&["breaking:gadget"]));
    assert_eq!(sorted.unexplained, vec![transitive("tiny", "1.0.0")]);
    assert_eq!(sorted.stale, Vec::new());
}

#[test]
fn sort_of_a_git_unit_applied_lists_it_applied_and_leaves_no_move_unexplained() {
    // Arrange
    let plan = plan_with_git();
    let current = CurrentState {
        lock: plan.lock.clone(),
        lines: lines(),
        git: git_at("ce129c85"),
    };

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(
        (
            sorted.applied,
            sorted.unexplained.len(),
            sorted.unexplained_git.len()
        ),
        (ids(&["git:gitcore"]), 0, 0)
    );
}

#[test]
fn sort_of_a_move_no_unit_explains_leaves_it_unexplained() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: lock(&[
                ("gadget", "0.13.1"),
                ("gizmo", "2.7.5"),
                ("libc", "0.2.180"),
                ("widget", "1.4.0"),
            ]),
            lines: lines(),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(
        sorted.unexplained,
        vec![moved("libc", "0.2.177", "0.2.180")]
    );
    assert_eq!(sorted.breaking, Vec::<String>::new());
}

#[test]
fn sort_of_a_requirement_edited_by_hand_is_stale_where_it_is() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: plan().lock,
            lines: lines_with(&[(21, "1.5")]),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(
        sorted
            .stale
            .iter()
            .map(|diagnostic| (diagnostic.code, diagnostic.at.clone()))
            .collect::<Vec<_>>(),
        vec![(DiagnosticCode::PlanStale, Some(at("Cargo.toml", 21)))]
    );
}

#[test]
fn sort_of_a_unit_applied_lists_it_applied_and_every_other_to_go() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: lock(&[
                ("gadget", "0.13.1"),
                ("gizmo", "2.7.6"),
                ("libc", "0.2.177"),
                ("widget", "1.4.1"),
            ]),
            lines: lines(),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(sorted.applied, ids(&["compatible:widget"]));
    assert_eq!(
        sorted.to_go,
        ids(&[
            "breaking:gadget",
            "toolchain:rust",
            "breaking:widget",
            "rest"
        ])
    );
    assert_eq!(sorted.unexplained, Vec::new());
}

#[test]
fn sort_of_an_edit_a_unit_writes_is_no_staleness() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: plan().lock,
            lines: lines_with(&[(9, "1.100.0")]),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(sorted.stale, Vec::new());
    assert_eq!(sorted.applied, ids(&["toolchain:rust"]));
}

#[test]
fn sort_of_the_workspace_the_plan_found_lists_every_unit_to_go() {
    // Arrange
    let (plan, current) = (
        plan(),
        CurrentState {
            lock: plan().lock,
            lines: lines(),
            git: Vec::new(),
        },
    );

    // Act
    let sorted = Sorting::new(&plan, &current).sort();

    // Assert
    assert_eq!(sorted.applied, Vec::<String>::new());
    assert_eq!(
        sorted.to_go,
        ids(&[
            "breaking:gadget",
            "toolchain:rust",
            "compatible:widget",
            "breaking:widget",
            "rest"
        ])
    );
    assert_eq!((sorted.unexplained, sorted.stale), (Vec::new(), Vec::new()));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use crate::support::policy_trees::tree;
use crate::support::recorded::base;
use crate::support::recorded::changed;
use crate::support::recorded::edge;
use crate::support::recorded::lines_with;
use crate::support::recorded::lock;
use crate::support::recorded::plan;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::planning::round::current_state::CurrentState;
use ripe4rust_core::planning::round::round_reader::RoundReader;

#[test]
fn read_of_a_changed_workspace_is_its_lock_and_lines_now() {
    // Arrange
    let mut files = changed(lock(&[("widget", "1.4.1")]), lines_with(&[(21, "1.5")]));
    files.edges = vec![edge(("gadget", "0.13.1"), ("widget", "1.4.1"))];

    // Act
    let read = RoundReader::new(&files).read(&plan());

    // Assert
    assert_eq!(
        read.map(|read| read.current),
        Ok(CurrentState {
            lock: lock(&[("widget", "1.4.1")]),
            lines: lines_with(&[(21, "1.5")]),
            git: Vec::new(),
            edges: vec![edge(("gadget", "0.13.1"), ("widget", "1.4.1"))],
        })
    );
}

#[test]
fn read_of_a_policy_in_error_stops_with_its_diagnostics() {
    // Arrange
    let mut files = base();
    files.policy = tree(vec![entry("young-dayz", PolicyValue::Integer(3), 32)]);

    // Act
    let read = RoundReader::new(&files).read(&plan());

    // Assert
    assert_eq!(
        read.map_err(|failure| {
            failure
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code)
                .collect::<Vec<_>>()
        })
        .err(),
        Some(vec![
            DiagnosticCode::PolicyUnknownKey,
            DiagnosticCode::GatesUndeclared
        ])
    );
}

#[test]
fn read_of_a_workspace_with_the_plan_s_fingerprint_is_the_plan_s_lock_and_lines() {
    // Arrange
    let mut files = base();
    files.lock = lock(&[("unread", "1.0.0")]);
    files.edges = vec![edge(("unread", "1.0.0"), ("widget", "1.4.0"))];
    let recorded = RecordedPlan {
        edges: Some(vec![edge(("gadget", "0.13.1"), ("widget", "1.4.0"))]),
        ..plan()
    };

    // Act
    let read = RoundReader::new(&files).read(&recorded);

    // Assert
    assert_eq!(
        read.map(|read| read.current),
        Ok(CurrentState {
            lock: plan().lock,
            lines: plan().lines,
            git: Vec::new(),
            edges: vec![edge(("gadget", "0.13.1"), ("widget", "1.4.0"))],
        })
    );
}

#[test]
fn read_passes_the_policy_s_warnings_on_and_its_pins() {
    // Arrange
    let mut files = base();
    files.policy = tree(vec![entry(
        "pins",
        PolicyValue::Table(vec![entry(
            "gizmo",
            PolicyValue::Table(vec![
                entry("reason", text("3 breaks the build"), 41),
                entry("review", text("2027-01-31"), 41),
            ]),
            41,
        )]),
        40,
    )]);

    // Act
    let read = RoundReader::new(&files).read(&plan()).expect("a reading");

    // Assert
    assert_eq!(
        read.policy
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        vec!["gizmo"]
    );
    assert_eq!(
        read.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::GatesUndeclared]
    );
}

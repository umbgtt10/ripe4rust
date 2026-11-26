// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::breaking;
use crate::support::plans::compatible;
use crate::support::plans::moved;
use crate::support::plans::plan;
use crate::support::plans::rest;
use crate::support::plans::toolchain;
use crate::support::reports::held;
use crate::support::reports::not_as_planned;
use cargo_ripe4rust::render::document_renderer::DocumentRenderer;
use cargo_ripe4rust::render::table::table_renderer::TableRenderer;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::guiding::guide::Guide;

#[test]
fn check_prints_the_report() {
    // Arrange & Act
    let text = TableRenderer.check(&not_as_planned());

    // Assert
    assert!(
        text.starts_with("check compatible:widget: not as planned\n"),
        "{text}"
    );
}

#[test]
fn failure_prints_each_diagnostic_on_a_line() {
    // Arrange
    let failure = PlanFailure {
        diagnostics: vec![DiagnosticCode::CargoDenyMissing.diagnose(
            None,
            String::from("cargo-deny is not installed"),
            String::from("install it"),
        )],
    };

    // Act
    let text = TableRenderer.failure(&failure);

    // Assert
    assert_eq!(
        text,
        "error[cargo-deny-missing] cargo-deny is not installed. Do: install it\n"
    );
}

#[test]
fn guide_prints_the_guide() {
    // Arrange & Act
    let text = TableRenderer.guide(&Guide::new("0.0.1").text());

    // Assert
    assert!(text.starts_with("ripe4rust 0.0.1: the guide\n"), "{text}");
}

#[test]
fn hold_prints_the_pins() {
    // Arrange & Act
    let text = TableRenderer.hold(&held(true));

    // Assert
    assert!(
        text.starts_with(
            "hold compatible:widget: under [workspace.metadata.ripe4rust.pins] at Cargo.toml:40\n"
        ),
        "{text}"
    );
}

#[test]
fn plan_marks_a_unit_whose_own_move_is_held() {
    // Arrange
    let mut held = compatible();
    held.unit.to = None;
    held.unit.moves = vec![moved("widget_builder", Some("4.6.6"), Some("4.6.7"))];
    held.unit.held = vec![moved("widget", Some("4.6.6"), Some("4.6.7"))];

    // Act
    let text = TableRenderer.plan(&plan(vec![held]));

    // Assert
    assert!(
        text.contains("widget (+1 transitive)  compatible  4.6.6     held      ripe"),
        "{text}"
    );
}

#[test]
fn plan_marks_an_urgent_unit() {
    // Arrange
    let mut urgent = compatible();
    urgent.decision.urgent = true;

    // Act
    let text = TableRenderer.plan(&plan(vec![urgent]));

    // Assert
    assert!(text.contains("ripe, urgent"));
}

#[test]
fn plan_prints_its_diagnostics_after_the_table() {
    // Arrange
    let mut plan = plan(Vec::new());
    plan.diagnostics = vec![DiagnosticCode::ToolchainUnpinned.diagnose(
        None,
        String::from("no channel is pinned"),
        String::from("nothing is needed"),
    )];

    // Act
    let text = TableRenderer.plan(&plan);

    // Assert
    assert!(text.ends_with(
        "outcome          reasons\n\nnotice[toolchain-unpinned] no channel is pinned. Do: nothing is needed\n"
    ));
}

#[test]
fn plan_prints_the_header_then_one_row_per_unit() {
    // Arrange
    let plan = plan(vec![compatible(), toolchain(), breaking(), rest()]);

    // Act
    let text = TableRenderer.plan(&plan);

    // Assert
    assert_eq!(
        text,
        [
            "date 2026-10-30, registry read 2026-10-30, workspace 9f2c41d0",
            "unit                    kind        from      to        outcome          reasons",
            "widget (+1 transitive)  compatible  4.6.6     4.6.7     ripe             compatible",
            "rust                    toolchain   1.99.1    1.100.0   ripe             toolchain; releases 2 versions held back",
            "gadget                  breaking    0.13.1    0.14.0    ask              reaches thumbv7em-none-eabihf; plan it: footprint 42 lines in 7 files",
            "the rest                compatible  --        --        ripe             2 moves no unit reaches",
            "",
        ]
        .join("\n")
    );
}

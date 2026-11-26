// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::breaking;
use crate::support::plans::compatible;
use crate::support::plans::plan;
use crate::support::plans::rest;
use crate::support::plans::toolchain;
use cargo_ripe4rust::render::table::table_renderer::TableRenderer;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;

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

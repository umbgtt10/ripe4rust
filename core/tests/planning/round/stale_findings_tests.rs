// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::at;
use crate::support::surveyed::written;
use crate::support::units::moved;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::round::line_change::LineChange;
use ripe4rust_core::planning::round::stale_findings::StaleFindings;

const AGAIN: &str =
    "plan again -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan";

#[test]
fn line_of_a_line_gone_is_stale_where_the_plan_found_it() {
    // Arrange
    let change = LineChange::Gone {
        then: written(at("Cargo.toml", 22), "gadget", "0.13"),
    };

    // Act
    let diagnostic = StaleFindings.line(&change);

    // Assert
    assert_eq!(
        diagnostic,
        DiagnosticCode::PlanStale.diagnose(
            Some(at("Cargo.toml", 22)),
            String::from(
                "`gadget` at Cargo.toml:22, which read `0.13`, is gone, and no unit of the plan removes it"
            ),
            String::from(AGAIN),
        )
    );
}

#[test]
fn line_of_a_line_new_is_stale_where_it_is() {
    // Arrange
    let change = LineChange::New {
        now: written(at("app/Cargo.toml", 4), "tiny", "1"),
    };

    // Act
    let diagnostic = StaleFindings.line(&change);

    // Assert
    assert_eq!(
        (diagnostic.at, diagnostic.what),
        (
            Some(at("app/Cargo.toml", 4)),
            String::from(
                "`tiny` at app/Cargo.toml:4 reads `1`, a line the plan did not find, and no unit of the plan writes it"
            )
        )
    );
}

#[test]
fn line_of_a_value_rewritten_is_stale_where_it_is_now() {
    // Arrange
    let change = LineChange::Rewritten {
        then: written(at("Cargo.toml", 21), "widget", "1.4"),
        now: written(at("Cargo.toml", 23), "widget", "1.5"),
    };

    // Act
    let diagnostic = StaleFindings.line(&change);

    // Assert
    assert_eq!(
        (diagnostic.at, diagnostic.what),
        (
            Some(at("Cargo.toml", 23)),
            String::from(
                "`widget` at Cargo.toml:23 reads `1.5`, where the plan found `1.4`, and no unit of the plan writes it"
            )
        )
    );
}

#[test]
fn lock_of_a_move_no_unit_makes_is_stale_in_the_lock() {
    // Arrange & Act
    let diagnostic: Diagnostic = StaleFindings.lock(&moved("libc", "0.2.177", "0.2.180"));

    // Assert
    assert_eq!(
        diagnostic,
        DiagnosticCode::PlanStale.diagnose(
            None,
            String::from(
                "in `Cargo.lock`, `libc` moved 0.2.177 -> 0.2.180, a move no unit of the plan makes"
            ),
            String::from(AGAIN),
        )
    );
}

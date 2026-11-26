// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::stopping::Stopping;

#[test]
fn at_errors_of_warnings_alone_goes_on_and_of_an_error_stops_with_all_found() {
    // Arrange
    let warning =
        DiagnosticCode::GatesUndeclared.diagnose(None, String::from("no gates"), String::new());
    let error =
        DiagnosticCode::PinWithoutReason.diagnose(None, String::from("no reason"), String::new());

    // Act
    let (warned, failed) = (vec![warning.clone()], vec![warning.clone(), error.clone()]);
    let stopped = [Stopping.at_errors(&warned), Stopping.at_errors(&failed)];

    // Assert
    assert_eq!(
        stopped,
        [
            Ok(()),
            Err(PlanFailure {
                diagnostics: vec![warning, error],
            }),
        ]
    );
}

#[test]
fn with_a_failure_is_what_was_found_and_the_failure_last() {
    // Arrange
    let warning =
        DiagnosticCode::GatesUndeclared.diagnose(None, String::from("no gates"), String::new());

    // Act
    let found = vec![warning.clone()];
    let stopped = Stopping.with(&found, &PortFailure::CurlMissing);

    // Assert
    assert_eq!(
        stopped,
        PlanFailure {
            diagnostics: vec![warning, PortFailure::CurlMissing.diagnostic()],
        }
    );
}

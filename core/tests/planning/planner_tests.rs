// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_ports::FakePorts;
use crate::support::surveyed::surveyed;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::planning::plan_writer::PlanWriter;
use ripe4rust_core::planning::planner::Planner;

#[test]
fn plan_of_the_fakes_is_the_plan_of_their_survey() {
    // Arrange
    let fakes = FakePorts::new();

    // Act
    let plan = Planner::new(fakes.ports()).plan();

    // Assert
    assert_eq!(plan, Ok(PlanWriter::new(&surveyed()).plan()));
}

#[test]
fn plan_without_cargo_deny_is_none() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.advisories.failure = Some(PortFailure::CargoDenyMissing);

    // Act
    let plan = Planner::new(fakes.ports()).plan();

    // Assert
    assert_eq!(
        plan.map_err(|failure| failure
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>()),
        Err(vec![DiagnosticCode::CargoDenyMissing])
    );
}

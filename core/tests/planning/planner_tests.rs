// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_ports::FakePorts;
use crate::support::surveyed::surveyed;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::scan::Scan;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::plan_writer::PlanWriter;
use ripe4rust_core::planning::planned_unit::PlannedUnit;
use ripe4rust_core::planning::planner::Planner;

fn names(members: &[&str]) -> Vec<String> {
    members.iter().map(|member| String::from(*member)).collect()
}

fn scanned(exposed_in: &[&str], unread_apis: &[&str]) -> Scan {
    let mut scan = surveyed().scan.expect("a scan");
    scan.uses[1].exposed_in = names(exposed_in);
    scan.unread_apis = names(unread_apis);
    scan
}

fn widget(scan: Option<Scan>) -> PlannedUnit {
    let mut fakes = FakePorts::new();
    for member in &mut fakes.metadata.workspace.members {
        member.rust_version = None;
    }
    fakes.scan.scan = scan;
    Planner::new(fakes.ports())
        .plan()
        .expect("a plan")
        .units
        .into_iter()
        .find(|planned| planned.unit.id.to_string() == "breaking:widget")
        .expect("the breaking unit")
}

#[test]
fn plan_of_a_scan_finding_the_crate_in_a_published_api_asks_it_with_its_work_list() {
    // Arrange & Act
    let planned = widget(Some(scanned(&["app-core"], &[])));

    // Assert
    assert_eq!(
        (planned.decision.outcome, planned.decision.rule),
        (Outcome::Ask, Some(8))
    );
    assert_eq!(
        planned.decision.reasons,
        names(&[
            "in app-core's public API",
            "plan it: footprint 3 lines in 2 files"
        ])
    );
    assert_eq!(
        planned
            .evidence
            .footprint
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>(),
        names(&["src/lib.rs:3", "src/lib.rs:9", "src/main.rs:2"])
    );
}

#[test]
fn plan_of_a_scan_finding_the_crate_in_no_public_api_takes_a_small_footprint_ripe() {
    // Arrange & Act
    let planned = widget(Some(scanned(&[], &[])));

    // Assert
    assert_eq!(
        (planned.decision.outcome, planned.decision.rule),
        (Outcome::Ripe, Some(11))
    );
    assert_eq!(
        planned.decision.reasons,
        names(&["footprint 3 lines in 2 files"])
    );
}

#[test]
fn plan_of_a_scan_that_could_not_read_a_published_api_asks_its_exposure() {
    // Arrange & Act
    let planned = widget(Some(scanned(&[], &["app-core"])));

    // Assert
    assert_eq!(planned.decision.rule, Some(8));
    assert_eq!(
        planned.decision.reasons,
        names(&[
            "exposure in app-core's public API not known",
            "plan it: footprint 3 lines in 2 files"
        ])
    );
}

#[test]
fn plan_of_a_scan_that_failed_as_a_whole_asks_with_neither_footprint_nor_exposure_known() {
    // Arrange & Act
    let planned = widget(None);

    // Assert
    assert_eq!(planned.decision.rule, Some(8));
    assert_eq!(
        planned.decision.reasons,
        names(&[
            "exposure in app-core's public API not known",
            "wait: footprint not known"
        ])
    );
    assert_eq!(planned.evidence.footprint, Vec::new());
}

#[test]
fn plan_of_sources_not_yet_scanned_asks_a_published_member_s_breaking_unit() {
    // Arrange
    let mut fakes = FakePorts::new();
    fakes.scan.scan = None;

    // Act
    let plan = Planner::new(fakes.ports()).plan().expect("a plan");

    // Assert
    let widget = plan
        .units
        .iter()
        .find(|planned| planned.unit.id.to_string() == "breaking:widget")
        .expect("the breaking unit");
    assert_eq!(widget.decision.rule, Some(8));
    assert_eq!(
        widget.decision.reasons.first().map(String::as_str),
        Some("exposure in app-core's public API not known")
    );
    assert_eq!(
        plan.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::SourceUnscanned]
    );
}

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

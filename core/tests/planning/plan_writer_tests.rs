// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::surveyed::surveyed;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_writer::PlanWriter;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::values::requirement::Requirement;

fn rows(plan: &Plan) -> Vec<(String, Outcome, Option<u8>)> {
    plan.units
        .iter()
        .map(|planned| {
            (
                planned.unit.id.to_string(),
                planned.decision.outcome,
                planned.decision.rule,
            )
        })
        .collect()
}

#[test]
fn plan_carries_the_survey_s_date_workspace_and_gates() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let plan = PlanWriter::new(&surveyed).plan();

    // Assert
    assert_eq!(plan.date, surveyed.today);
    assert_eq!(plan.registry_read, surveyed.registry_read);
    assert_eq!(plan.workspace.fingerprint, "9f2c41d0");
    assert_eq!(
        plan.workspace.lock,
        vec![
            String::from("gadget 0.13.1"),
            String::from("gizmo 2.7.5"),
            String::from("libc 0.2.177"),
            String::from("widget 1.4.0"),
        ]
    );
    assert_eq!(plan.gates, surveyed.policy.gates);
}

#[test]
fn plan_judges_every_unit_in_plan_order() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let plan = PlanWriter::new(&surveyed).plan();

    // Assert
    assert_eq!(
        rows(&plan),
        vec![
            (String::from("breaking:gadget"), Outcome::Ripening, Some(9)),
            (String::from("toolchain:rust"), Outcome::Ripe, Some(10)),
            (String::from("compatible:widget"), Outcome::Ripe, Some(10)),
            (String::from("breaking:widget"), Outcome::Ask, Some(8)),
            (String::from("rest"), Outcome::Ripe, Some(10)),
        ]
    );
}

#[test]
fn plan_names_a_crate_the_lock_holds_twice_at_its_version() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.workspace.lock.push(LockedPackage {
        name: String::from("gizmo"),
        version: version("1.0.0"),
        dependencies: Vec::new(),
    });
    surveyed.policy.pins = Vec::new();

    // Act
    let plan = PlanWriter::new(&surveyed).plan();

    // Assert
    let split = plan.units[2].steps.clone().expect("steps").split;
    assert_eq!(
        split[1].apply,
        vec![Step::Run {
            command: String::from("cargo update -p gizmo@2.7.5 --precise 2.7.6"),
        }]
    );
}

#[test]
fn plan_puts_an_advisory_on_the_unit_that_clears_it_and_rows_the_rest() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.advisories = vec![
        Advisory {
            id: String::from("RUSTSEC-2026-0190"),
            name: String::from("gizmo"),
            version: version("2.7.5"),
            patched: vec![Requirement::parse(">=2.7.6").expect("a requirement")],
            unaffected: Vec::new(),
        },
        Advisory {
            id: String::from("RUSTSEC-2026-0002"),
            name: String::from("gadget"),
            version: version("0.13.1"),
            patched: Vec::new(),
            unaffected: Vec::new(),
        },
    ];

    // Act
    let plan = PlanWriter::new(&surveyed).plan();

    // Assert
    let rows = rows(&plan);
    assert_eq!(
        rows[2],
        (String::from("compatible:widget"), Outcome::Ripe, Some(2))
    );
    assert_eq!(
        rows[5],
        (
            String::from("advisory:RUSTSEC-2026-0002"),
            Outcome::Ask,
            None
        )
    );
}

#[test]
fn plan_records_every_line_a_unit_can_edit_as_it_was_written() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let plan = PlanWriter::new(&surveyed).plan();

    // Assert
    assert_eq!(plan.workspace.lines, surveyed.written);
}

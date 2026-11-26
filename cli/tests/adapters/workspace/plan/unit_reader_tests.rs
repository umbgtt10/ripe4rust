// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::plan::plan_text::PlanText;
use cargo_ripe4rust::adapters::workspace::plan::unit_reader::UnitReader;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_unit::RecordedUnit;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::version::Version;
use serde_json::Value;
use serde_json::from_str;
use serde_json::json;

const PLAN: &str = include_str!("../../../fixtures/workspace/plan.json");

fn recorded(index: usize) -> Value {
    from_str::<Value>(PLAN).expect("a plan")["units"][index].clone()
}

fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

#[test]
fn unit_of_a_breaking_unit_is_its_moves_its_edits_and_its_revert() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let unit = UnitReader::new(&text).unit(&recorded(1));

    // Assert
    assert_eq!(
        unit,
        Ok(RecordedUnit {
            id: String::from("breaking:itertools"),
            kind: UnitKind::Breaking,
            name: Some(String::from("itertools")),
            members: vec![String::from("itertools")],
            from: Some(version("0.10.0")),
            to: Some(version("0.15.0")),
            moves: vec![LockMove {
                name: String::from("itertools"),
                from: Some(version("0.10.0")),
                to: Some(version("0.15.0")),
            }],
            urgent: false,
            edits: vec![WrittenLine {
                at: Location {
                    file: String::from("Cargo.toml"),
                    line: 12,
                },
                key: String::from("itertools"),
                value: String::from("0.15.0"),
            }],
            splits: Vec::new(),
            revert: Some(String::from("git restore -- Cargo.lock Cargo.toml")),
        })
    );
}

#[test]
fn unit_of_a_compatible_unit_is_the_ids_of_its_split() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let unit = UnitReader::new(&text).unit(&recorded(0));

    // Assert
    assert_eq!(
        unit.map(|unit| unit.splits),
        Ok(vec![
            String::from("compatible:itertools/either"),
            String::from("compatible:itertools/itertools"),
        ])
    );
}

#[test]
fn unit_of_a_kind_it_does_not_know_is_unreadable() {
    // Arrange
    let text = PlanText::new("plan.json");
    let mut unit = recorded(2);
    unit["kind"] = json!("major");

    // Act
    let read = UnitReader::new(&text).unit(&unit);

    // Assert
    assert_eq!(
        read,
        Err(PortFailure::PlanUnreadable {
            file: String::from("plan.json"),
            detail: String::from("`compatible:itoa` is of a kind ripe4rust does not know, `major`"),
        })
    );
}

#[test]
fn unit_of_an_advisory_row_has_no_revert_and_is_urgent() {
    // Arrange
    let text = PlanText::new("plan.json");
    let row = json!({
        "id": "advisory:RUSTSEC-2026-0001",
        "kind": "advisory",
        "crate": "gizmo",
        "members": ["gizmo"],
        "from": "2.7.5",
        "to": null,
        "moves": [],
        "urgent": true,
        "steps": null
    });

    // Act
    let unit = UnitReader::new(&text).unit(&row);

    // Assert
    assert_eq!(
        unit.map(|unit| (unit.kind, unit.urgent, unit.revert)),
        Ok((UnitKind::Advisory, true, None))
    );
}

#[test]
fn unit_that_is_no_unit_is_unreadable() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = UnitReader::new(&text).unit(&json!({ "id": "compatible:itoa" }));

    // Assert
    assert!(
        matches!(&read, Err(PortFailure::PlanUnreadable { detail, .. }) if detail.starts_with("missing field")),
        "{read:?}"
    );
}

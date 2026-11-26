// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::units::plan_order::PlanOrder;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

fn rest() -> Unit {
    Unit {
        id: UnitId::rest(),
        kind: UnitKind::Compatible,
        name: None,
        members: Vec::new(),
        from: None,
        to: None,
        moves: vec![moved("libc", "0.2.177", "0.2.178")],
        held: Vec::new(),
        git: None,
    }
}

fn row() -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Advisory, "RUSTSEC-2026-0002"),
        kind: UnitKind::Advisory,
        name: Some(String::from("atty")),
        members: Vec::new(),
        from: None,
        to: None,
        moves: Vec::new(),
        held: Vec::new(),
        git: None,
    }
}

fn toolchain() -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: None,
        to: None,
        moves: Vec::new(),
        held: Vec::new(),
        git: None,
    }
}

#[test]
fn sorted_takes_names_in_order_compatible_first_then_the_rest_then_the_rows() {
    // Arrange
    let units = vec![
        row(),
        rest(),
        breaking("syn", "2.0.119", "3.0.9"),
        toolchain(),
        compatible("syn", vec![moved("syn", "2.0.119", "2.0.120")]),
        compatible("anyhow", vec![moved("anyhow", "1.0.102", "1.0.104")]),
    ];

    // Act
    let sorted = PlanOrder.sorted(units);

    // Assert
    assert_eq!(
        sorted
            .iter()
            .map(|unit| unit.id.to_string())
            .collect::<Vec<_>>(),
        vec![
            String::from("compatible:anyhow"),
            String::from("toolchain:rust"),
            String::from("compatible:syn"),
            String::from("breaking:syn"),
            String::from("rest"),
            String::from("advisory:RUSTSEC-2026-0002"),
        ]
    );
}

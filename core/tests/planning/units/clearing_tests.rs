// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::planning::units::clearing::Clearing;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::values::requirement::Requirement;

fn advisory(id: &str, name: &str, at: &str, patched: &str) -> Concern {
    Concern::Advisory(Advisory {
        id: String::from(id),
        name: String::from(name),
        version: version(at),
        patched: vec![Requirement::parse(patched).expect("a requirement")],
        unaffected: Vec::new(),
    })
}

#[test]
fn assign_gives_a_breaking_unit_the_concern_its_target_clears() {
    // Arrange
    let concern = advisory("RUSTSEC-2026-0001", "url", "2.5.8", ">=3");
    let units = vec![
        compatible("url", vec![moved("url", "2.5.8", "2.5.9")]),
        breaking("url", "2.5.8", "3.0.0"),
    ];

    // Act
    let assigned = Clearing::new(vec![concern.clone()]).assign(&units);

    // Assert
    assert_eq!(assigned, vec![Vec::new(), vec![concern]]);
}

#[test]
fn assign_gives_a_concern_to_the_first_unit_that_clears_it() {
    // Arrange
    let concern = advisory("RUSTSEC-2026-0592", "url", "2.5.8", ">=2.5.9");
    let units = vec![
        compatible(
            "reqwest",
            vec![
                moved("reqwest", "0.12.1", "0.12.2"),
                moved("url", "2.5.8", "2.5.9"),
            ],
        ),
        breaking("url", "2.5.8", "3.0.0"),
    ];

    // Act
    let assigned = Clearing::new(vec![concern.clone()]).assign(&units);

    // Assert
    assert_eq!(assigned, vec![vec![concern], Vec::new()]);
}

#[test]
fn rows_are_the_concerns_no_unit_clears() {
    // Arrange
    let cleared = advisory("RUSTSEC-2026-0592", "url", "2.5.8", ">=2.5.9");
    let uncleared = advisory("RUSTSEC-2026-0002", "atty", "0.2.14", ">=9");
    let units = vec![compatible("url", vec![moved("url", "2.5.8", "2.5.9")])];

    // Act
    let rows = Clearing::new(vec![cleared, uncleared]).rows(&units);

    // Assert
    assert_eq!(
        rows.iter()
            .map(|row| row.id.to_string())
            .collect::<Vec<_>>(),
        vec![String::from("advisory:RUSTSEC-2026-0002")]
    );
}

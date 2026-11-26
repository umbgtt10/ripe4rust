// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::moved;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::requirement::Requirement;

fn advisory() -> Concern {
    Concern::Advisory(Advisory {
        id: String::from("RUSTSEC-2026-0592"),
        name: String::from("url"),
        version: version("2.5.8"),
        patched: vec![Requirement::parse(">=2.5.9").expect("a requirement")],
        unaffected: vec![Requirement::parse("<2.5.0").expect("a requirement")],
    })
}

fn yanked() -> Concern {
    Concern::Yanked {
        name: String::from("spin"),
        version: version("0.10.0"),
    }
}

#[test]
fn ignore_entry_of_a_yanked_release_names_the_crate_at_its_version() {
    // Arrange & Act
    let entry = yanked().ignore_entry();

    // Assert
    assert_eq!(
        entry,
        "{ crate = \"spin@0.10.0\", reason = \"<the person's words>\" }"
    );
}

#[test]
fn ignore_entry_of_an_advisory_names_its_id() {
    // Arrange & Act
    let entry = advisory().ignore_entry();

    // Assert
    assert_eq!(
        entry,
        "{ id = \"RUSTSEC-2026-0592\", reason = \"<the person's words>\" }"
    );
}

#[test]
fn is_cleared_by_a_move_of_another_crate_is_false() {
    // Arrange & Act
    let cleared = advisory().is_cleared_by(&moved("idna", "2.5.8", "2.5.9"));

    // Assert
    assert!(!cleared);
}

#[test]
fn is_cleared_by_a_move_off_a_yanked_release_is_true() {
    // Arrange & Act
    let cleared = yanked().is_cleared_by(&moved("spin", "0.10.0", "0.10.1"));

    // Assert
    assert!(cleared);
}

#[test]
fn is_cleared_by_a_move_to_a_patched_version_is_true() {
    // Arrange & Act
    let cleared = advisory().is_cleared_by(&moved("url", "2.5.8", "2.5.9"));

    // Assert
    assert!(cleared);
}

#[test]
fn is_cleared_by_a_move_to_an_unaffected_version_is_true() {
    // Arrange & Act
    let cleared = advisory().is_cleared_by(&moved("url", "2.5.8", "2.4.0"));

    // Assert
    assert!(cleared);
}

#[test]
fn is_cleared_by_a_move_to_another_affected_version_is_false() {
    // Arrange & Act
    let cleared = advisory().is_cleared_by(&moved("url", "2.5.8", "2.5.8"));

    // Assert
    assert!(!cleared);
}

#[test]
fn label_of_a_yanked_release_names_it() {
    // Arrange & Act
    let label = yanked().label();

    // Assert
    assert_eq!(label, "the yanked spin 0.10.0");
}

#[test]
fn label_of_an_advisory_is_its_id() {
    // Arrange & Act
    let label = advisory().label();

    // Assert
    assert_eq!(label, "RUSTSEC-2026-0592");
}

#[test]
fn name_of_a_concern_is_its_crate() {
    // Arrange & Act
    let names = [advisory().name().to_string(), yanked().name().to_string()];

    // Assert
    assert_eq!(names, [String::from("url"), String::from("spin")]);
}

#[test]
fn row_of_a_yanked_release_goes_by_crate_at_version() {
    // Arrange & Act
    let row = yanked().row();

    // Assert
    assert_eq!(row.id.to_string(), "advisory:spin@0.10.0");
}

#[test]
fn row_of_an_advisory_is_an_advisory_row_for_its_crate() {
    // Arrange & Act
    let row = advisory().row();

    // Assert
    assert_eq!(
        row,
        Unit {
            id: UnitId::of(UnitKind::Advisory, "RUSTSEC-2026-0592"),
            kind: UnitKind::Advisory,
            name: Some(String::from("url")),
            members: Vec::new(),
            from: Some(version("2.5.8")),
            to: None,
            moves: Vec::new(),
            held: Vec::new(),
        }
    );
}

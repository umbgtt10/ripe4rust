// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::planning::writing::summary_writer::SummaryWriter;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn summary_of_a_breaking_unit_migrates_it() {
    // Arrange & Act
    let summary = SummaryWriter.summary(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(summary, "Migrate widget to 2.1.0");
}

#[test]
fn summary_of_a_compatible_unit_alone_names_its_version() {
    // Arrange & Act
    let summary = SummaryWriter.summary(
        &compatible("anyhow", vec![moved("anyhow", "1.0.102", "1.0.104")]),
        &[],
    );

    // Assert
    assert_eq!(summary, "Move anyhow to 1.0.104");
}

#[test]
fn summary_of_a_compatible_unit_counts_what_it_pulls_in() {
    // Arrange
    let unit = compatible(
        "clap",
        vec![
            moved("clap", "4.6.6", "4.6.7"),
            moved("clap_builder", "4.6.6", "4.6.7"),
            moved("clap_lex", "0.7.5", "0.7.6"),
            moved("anstyle", "1.0.10", "1.0.11"),
        ],
    );

    // Act
    let summary = SummaryWriter.summary(&unit, &[]);

    // Assert
    assert_eq!(summary, "Move clap to 4.6.7, with 3 crates it pulls in");
}

#[test]
fn summary_of_a_dependency_that_does_not_move_itself_counts_what_it_pulls_in() {
    // Arrange
    let mut unit = compatible("tokio", vec![moved("mio", "1.0.4", "1.0.5")]);
    unit.from = Some(version("1.48.0"));
    unit.to = Some(version("1.48.0"));

    // Act
    let summary = SummaryWriter.summary(&unit, &[]);

    // Assert
    assert_eq!(summary, "Move 1 crate tokio pulls in");
}

#[test]
fn summary_of_a_group_names_its_members() {
    // Arrange
    let mut unit = compatible(
        "serde",
        vec![
            moved("serde", "1.0.228", "1.0.229"),
            moved("serde_derive", "1.0.228", "1.0.229"),
        ],
    );
    unit.members.push(String::from("serde_derive"));

    // Act
    let summary = SummaryWriter.summary(&unit, &[]);

    // Assert
    assert_eq!(summary, "Move serde with serde_derive to 1.0.229");
}

#[test]
fn summary_of_an_advisory_row_resolves_it() {
    // Arrange
    let concern = Concern::Yanked {
        name: String::from("spin"),
        version: version("0.10.0"),
    };

    // Act
    let summary = SummaryWriter.summary(&concern.row(), &[concern]);

    // Assert
    assert_eq!(summary, "Resolve the yanked spin 0.10.0 in spin 0.10.0");
}

#[test]
fn summary_of_the_rest_counts_its_moves() {
    // Arrange
    let rest = Unit {
        id: UnitId::rest(),
        kind: UnitKind::Compatible,
        name: None,
        members: Vec::new(),
        from: None,
        to: None,
        moves: vec![
            moved("libc", "0.2.177", "0.2.178"),
            moved("ryu", "1.0.20", "1.0.21"),
        ],
        held: Vec::new(),
    };

    // Act
    let summary = SummaryWriter.summary(&rest, &[]);

    // Assert
    assert_eq!(summary, "Move 2 crates no unit reaches");
}

#[test]
fn summary_of_the_toolchain_moves_it() {
    // Arrange
    let mut unit = breaking("rust", "1.99.1", "1.100.0");
    unit.kind = UnitKind::Toolchain;

    // Act
    let summary = SummaryWriter.summary(&unit, &[]);

    // Assert
    assert_eq!(summary, "Move the toolchain to 1.100.0");
}

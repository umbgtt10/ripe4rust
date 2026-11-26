// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use crate::support::units::dependency;
use crate::support::units::moved;
use crate::support::units::source;
use ripe4rust_core::planning::units::claimant::Claimant;
use ripe4rust_core::planning::units::dependency_moves::DependencyMoves;
use ripe4rust_core::planning::units::source_moves::SourceMoves;

fn both() -> [Claimant; 2] {
    [
        Claimant::Dependency(DependencyMoves {
            dependency: dependency("itoa", "1.0.10"),
            moves: vec![moved("itoa", "1.0.10", "1.0.19")],
        }),
        Claimant::Source(SourceMoves {
            source: source("9625c3d2", "ce129c85"),
            name: String::from("gitcore"),
            version: version("0.1.0"),
            crates: vec![String::from("gitcore")],
            moves: vec![moved("ryu", "1.0.1", "1.0.2")],
        }),
    ]
}

#[test]
fn moves_of_a_claimant_are_its_dependency_s_or_its_source_s() {
    // Arrange
    let [dependency, source] = both();

    // Act
    let moves = [dependency.moves(), source.moves()];

    // Assert
    assert_eq!(
        moves,
        [
            &[moved("itoa", "1.0.10", "1.0.19")][..],
            &[moved("ryu", "1.0.1", "1.0.2")][..]
        ]
    );
}

#[test]
fn name_of_a_claimant_is_its_dependency_s_or_its_source_s_crate() {
    // Arrange
    let [dependency, source] = both();

    // Act
    let names = [dependency.name(), source.name()];

    // Assert
    assert_eq!(names, ["itoa", "gitcore"]);
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::release;
use crate::support::units::breaking;
use crate::support::units::dependency;
use ripe4rust_core::planning::units::breaking_units::BreakingUnits;
use ripe4rust_core::planning::units::choice::histories::Histories;
use ripe4rust_core::planning::units::choice::maturity::Maturity;
use ripe4rust_core::planning::units::choice::release_history::ReleaseHistory;
use ripe4rust_core::values::version::Version;

fn histories() -> Histories {
    Histories::new(vec![
        (
            String::from("syn"),
            ReleaseHistory::new(
                vec![
                    release("2.0.119", "2026-01-01"),
                    release("3.0.0", "2026-06-01"),
                    release("3.0.8", "2026-07-01"),
                    release("3.0.9", "2026-08-01"),
                ],
                Version::line,
            ),
        ),
        (
            String::from("clap"),
            ReleaseHistory::new(vec![release("4.6.7", "2026-10-01")], Version::line),
        ),
    ])
}

#[test]
fn build_of_a_crate_the_index_does_not_know_offers_none() {
    // Arrange
    let histories = histories();
    let maturity = Maturity::new(date("2026-11-26"), 30, 3, 6);
    let toolchain = Version::new(1, 99, 1);

    // Act
    let units = BreakingUnits::new(&histories, &maturity, &toolchain)
        .build(&[dependency("tokio", "1.48.0")]);

    // Assert
    assert_eq!(units, Vec::new());
}

#[test]
fn build_offers_a_unit_for_each_dependency_with_a_newer_line() {
    // Arrange
    let histories = histories();
    let maturity = Maturity::new(date("2026-11-26"), 30, 3, 6);
    let toolchain = Version::new(1, 99, 1);
    let dependencies = vec![dependency("clap", "4.6.6"), dependency("syn", "2.0.119")];

    // Act
    let units = BreakingUnits::new(&histories, &maturity, &toolchain).build(&dependencies);

    // Assert
    assert_eq!(units, vec![breaking("syn", "2.0.119", "3.0.9")]);
}

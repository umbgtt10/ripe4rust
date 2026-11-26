// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::releases::yanked;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::added;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::downgrade::Downgrade;
use ripe4rust_core::planning::judging::facts::downgrade_finder::DowngradeFinder;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::surveyed::Surveyed;

#[test]
fn of_a_unit_moving_crates_back_names_each_and_whether_it_leaves_a_yanked_release() {
    // Arrange
    let mut releases = surveyed().releases;
    releases.push((
        String::from("crypto-common"),
        vec![
            release("0.1.6", "2022-08-01"),
            release("0.1.7", "2025-11-01"),
        ],
    ));
    releases.push((
        String::from("memchr"),
        vec![
            release("2.5.0", "2022-04-01"),
            yanked("2.7.0", "2023-12-01"),
        ],
    ));
    let surveyed = Surveyed {
        releases,
        ..surveyed()
    };
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let unit = compatible(
        "widget",
        vec![
            moved("widget", "1.4.0", "1.4.1"),
            moved("crypto-common", "0.1.7", "0.1.6"),
            added("libc", "0.2.178"),
            moved("memchr", "2.7.0", "2.5.0"),
        ],
    );

    // Act
    let downgrades = DowngradeFinder::new(&context).of(&unit);

    // Assert
    assert_eq!(
        downgrades,
        vec![
            Downgrade {
                name: String::from("crypto-common"),
                from: version("0.1.7"),
                to: version("0.1.6"),
                yanked: false,
            },
            Downgrade {
                name: String::from("memchr"),
                from: version("2.7.0"),
                to: version("2.5.0"),
                yanked: true,
            },
        ]
    );
}

#[test]
fn of_a_unit_moving_every_crate_forward_is_none() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let downgrades = DowngradeFinder::new(&context).of(&compatible(
        "widget",
        vec![moved("widget", "1.4.0", "1.4.1")],
    ));

    // Assert
    assert_eq!(downgrades, Vec::new());
}

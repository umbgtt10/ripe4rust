// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::at;
use crate::support::surveyed::surveyed;
use ripe4rust_core::inputs::source_scan::unfollowed_include::UnfollowedInclude;
use ripe4rust_core::planning::judging::facts::footprints::Footprints;
use ripe4rust_core::planning::judging::footprint::Footprint;

#[test]
fn footprint_counts_lines_and_files() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let footprint = Footprints::new(surveyed.scan.as_ref(), &surveyed.workspace.members)
        .footprint(&[String::from("widget")]);

    // Assert
    assert_eq!(
        footprint,
        Some(Footprint {
            lines: 3,
            files: 2,
            complete: true
        })
    );
}

#[test]
fn footprint_of_a_crate_a_member_including_code_unread_uses_is_a_floor() {
    // Arrange
    let mut surveyed = surveyed();
    if let Some(scan) = surveyed.scan.as_mut() {
        scan.unfollowed = vec![UnfollowedInclude {
            member: String::from("app"),
            at: at("app/build.rs", 3),
        }];
    }
    let footprints = Footprints::new(surveyed.scan.as_ref(), &surveyed.workspace.members);

    // Act
    let footprint = [
        footprints.footprint(&[String::from("gadget")]),
        footprints.footprint(&[String::from("libc")]),
    ];

    // Assert
    assert_eq!(
        footprint,
        [
            Some(Footprint {
                lines: 1,
                files: 1,
                complete: false
            }),
            Some(Footprint {
                lines: 0,
                files: 0,
                complete: true
            }),
        ]
    );
}

#[test]
fn footprint_of_sources_not_yet_scanned_is_unknown() {
    // Arrange & Act
    let footprint = Footprints::new(None, &[]).footprint(&[String::from("widget")]);

    // Assert
    assert_eq!(footprint, None);
}

#[test]
fn locations_of_a_group_are_its_members_together_in_order() {
    // Arrange
    let surveyed = surveyed();

    // Act
    let locations = Footprints::new(surveyed.scan.as_ref(), &surveyed.workspace.members)
        .locations(&[String::from("widget"), String::from("gadget")]);

    // Assert
    assert_eq!(
        locations,
        vec![
            at("src/lib.rs", 3),
            at("src/lib.rs", 9),
            at("src/main.rs", 2),
            at("src/net.rs", 4)
        ]
    );
}

#[test]
fn locations_of_sources_not_yet_scanned_are_none() {
    // Arrange & Act
    let locations = Footprints::new(None, &[]).locations(&[String::from("widget")]);

    // Assert
    assert_eq!(locations, Vec::new());
}

#[test]
fn unfollowed_of_a_crate_are_the_includes_unread_in_the_members_using_it() {
    // Arrange
    let mut surveyed = surveyed();
    if let Some(scan) = surveyed.scan.as_mut() {
        scan.unfollowed = vec![
            UnfollowedInclude {
                member: String::from("app-core"),
                at: at("app-core/src/wire.rs", 2),
            },
            UnfollowedInclude {
                member: String::from("app"),
                at: at("app/src/main.rs", 7),
            },
        ];
    }
    let footprints = Footprints::new(surveyed.scan.as_ref(), &surveyed.workspace.members);

    // Act
    let unfollowed = [
        footprints.unfollowed(&[String::from("widget")]),
        footprints.unfollowed(&[String::from("gadget")]),
    ];

    // Assert
    assert_eq!(
        unfollowed,
        [
            vec![at("app-core/src/wire.rs", 2), at("app/src/main.rs", 7)],
            vec![at("app/src/main.rs", 7)],
        ]
    );
}

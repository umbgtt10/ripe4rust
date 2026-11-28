// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::version;
use crate::support::surveyed::at;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::inputs::source_scan::unfollowed_include::UnfollowedInclude;
use ripe4rust_core::planning::judging::dependent::Dependent;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::units::choice::line_stats::LineStats;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::writing::evidence::Evidence;
use ripe4rust_core::planning::writing::evidence_writer::EvidenceWriter;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::requirement::Requirement;

#[test]
fn evidence_of_a_breaking_unit_a_dependent_holds_back_names_the_dependent() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.workspace.lock.push(LockedPackage {
        name: String::from("frame"),
        version: version("0.4.0"),
        dependencies: vec![ResolvedDependency {
            name: String::from("widget"),
            version: version("1.4.0"),
            requirement: Requirement::parse("^1.4").expect("a requirement"),
        }],
    });
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let evidence =
        EvidenceWriter::new(&context).evidence(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(
        evidence.held_by,
        vec![Dependent {
            name: String::from("frame"),
            version: version("0.4.0"),
            requires: String::from("widget"),
            requirement: Requirement::parse("^1.4").expect("a requirement"),
            in_use: version("1.4.0"),
            target: version("2.1.0"),
        }]
    );
}

#[test]
fn evidence_of_a_breaking_unit_carries_its_work_list() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let evidence =
        EvidenceWriter::new(&context).evidence(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(
        evidence,
        Evidence {
            line_in_use: Some(LineStats {
                line: Line {
                    major: 1,
                    minor: None
                },
                first_release: Some(date("2024-01-10")),
                last_release: Some(date("2026-10-01")),
                releases: 3,
            }),
            rust_version: Some(PartialVersion::parse("1.88").expect("a rust-version")),
            advisories: Vec::new(),
            yanked_in_use: false,
            footprint: vec![
                at("src/lib.rs", 3),
                at("src/lib.rs", 9),
                at("src/main.rs", 2)
            ],
            held_back: Vec::new(),
            held: Vec::new(),
            downgrades: Vec::new(),
            unfollowed_includes: Vec::new(),
            held_by: Vec::new(),
        }
    );
}

#[test]
fn evidence_of_a_breaking_unit_names_the_includes_its_footprint_could_not_follow() {
    // Arrange
    let mut surveyed = surveyed();
    if let Some(scan) = surveyed.scan.as_mut() {
        scan.unfollowed = vec![UnfollowedInclude {
            member: String::from("app"),
            at: at("app/src/wire.rs", 2),
        }];
    }
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let evidence = [
        EvidenceWriter::new(&context).evidence(&breaking("gadget", "0.13.1", "0.14.0"), &[]),
        EvidenceWriter::new(&context).evidence(
            &compatible("gadget", vec![moved("gadget", "0.13.0", "0.13.1")]),
            &[],
        ),
    ];

    // Assert
    assert_eq!(
        evidence.map(|evidence| evidence.unfollowed_includes),
        [vec![at("app/src/wire.rs", 2)], Vec::new()]
    );
}

#[test]
fn evidence_of_a_compatible_unit_names_what_it_clears_and_holds() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = compatible("widget", vec![moved("widget", "1.4.0", "1.4.1")]);
    unit.held = vec![moved("gizmo", "2.7.5", "2.7.6")];
    let concerns = [
        Concern::Advisory(Advisory {
            id: String::from("RUSTSEC-2026-0190"),
            name: String::from("widget"),
            version: version("1.4.0"),
            patched: Vec::new(),
            unaffected: Vec::new(),
        }),
        Concern::Yanked {
            name: String::from("widget"),
            version: version("1.4.0"),
        },
    ];

    // Act
    let evidence = EvidenceWriter::new(&context).evidence(&unit, &concerns);

    // Assert
    assert_eq!(evidence.advisories, vec![String::from("RUSTSEC-2026-0190")]);
    assert!(evidence.yanked_in_use);
    assert_eq!(evidence.held, vec![moved("gizmo", "2.7.5", "2.7.6")]);
    assert_eq!(evidence.footprint, Vec::new());
}

#[test]
fn evidence_of_a_unit_moving_a_crate_back_lists_the_move() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let unit = compatible(
        "widget",
        vec![
            moved("widget", "1.4.0", "1.4.1"),
            moved("gizmo", "2.7.6", "2.7.5"),
        ],
    );

    // Act
    let evidence = EvidenceWriter::new(&context).evidence(&unit, &[]);

    // Assert
    assert_eq!(evidence.downgrades, vec![moved("gizmo", "2.7.6", "2.7.5")]);
}

#[test]
fn evidence_of_a_unit_whose_crate_stays_names_the_rust_version_in_use() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = compatible("widget", vec![moved("gizmo", "2.7.5", "2.7.6")]);
    unit.from = Some(version("2.1.0"));

    // Act
    let evidence = EvidenceWriter::new(&context).evidence(&unit, &[]);

    // Assert
    assert_eq!(
        evidence.rust_version,
        Some(PartialVersion::parse("1.88").expect("a rust-version"))
    );
}

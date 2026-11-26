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
use ripe4rust_core::planning::choice::line_stats::LineStats;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::writing::evidence::Evidence;
use ripe4rust_core::planning::writing::evidence_writer::EvidenceWriter;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::partial_version::PartialVersion;

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
        }
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

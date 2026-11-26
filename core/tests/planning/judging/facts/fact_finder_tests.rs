// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::version;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::downgrade::Downgrade;
use ripe4rust_core::planning::judging::facts::fact_finder::FactFinder;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn facts_of_a_breaking_unit_gather_what_the_rules_read() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let facts = FactFinder::new(&context).facts(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(facts.kind, UnitKind::Breaking);
    assert_eq!(facts.name, "widget");
    assert_eq!(facts.rest, None);
    assert_eq!(facts.exposed_in, vec![String::from("app-core")]);
    assert_eq!(facts.raises.len(), 2);
    assert_eq!(facts.lines_behind, 1);
    assert_eq!(facts.footprint, Some(Footprint { lines: 3, files: 2 }));
    assert_eq!(facts.gates, 2);
    assert_eq!(facts.freeze, None);
    assert!(!facts.fully_held);
    assert!(facts.newer_active);
}

#[test]
fn facts_of_a_freeze_ending_today_carry_none() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.policy.freeze_until = Some(date("2026-11-26"));
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let facts = FactFinder::new(&context).facts(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(facts.freeze, None);
}

#[test]
fn facts_of_a_unit_clearing_a_concern_name_it() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let concern = Concern::Yanked {
        name: String::from("gizmo"),
        version: version("2.7.5"),
    };

    // Act
    let facts = FactFinder::new(&context).facts(
        &compatible("widget", vec![moved("gizmo", "2.7.5", "2.7.6")]),
        &[concern],
    );

    // Assert
    assert_eq!(facts.concerns, vec![String::from("the yanked gizmo 2.7.5")]);
}

#[test]
fn facts_of_a_unit_moving_a_crate_back_name_the_move() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let facts = FactFinder::new(&context).facts(
        &compatible("widget", vec![moved("gizmo", "2.7.6", "2.7.5")]),
        &[],
    );

    // Assert
    assert_eq!(
        facts.downgrades,
        vec![Downgrade {
            name: String::from("gizmo"),
            from: version("2.7.6"),
            to: version("2.7.5"),
            yanked: false,
        }]
    );
}

#[test]
fn facts_of_an_open_freeze_carry_its_end() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.policy.freeze_until = Some(date("2027-01-15"));
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let facts = FactFinder::new(&context).facts(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(facts.freeze, Some(date("2027-01-15")));
}

#[test]
fn facts_of_sources_not_yet_scanned_know_no_footprint_and_no_exposure() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.scan = None;
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let facts = FactFinder::new(&context).facts(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(facts.footprint, None);
    assert_eq!(facts.exposed_in, Vec::<String>::new());
    assert_eq!(facts.exposure_unknown_in, vec![String::from("app-core")]);
}

#[test]
fn facts_of_the_rest_count_its_moves() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let rest = Unit {
        id: UnitId::rest(),
        kind: UnitKind::Compatible,
        name: None,
        members: Vec::new(),
        from: None,
        to: None,
        moves: vec![moved("libc", "0.2.177", "0.2.178")],
        held: Vec::new(),
    };

    // Act
    let facts = FactFinder::new(&context).facts(&rest, &[]);

    // Assert
    assert_eq!(facts.rest, Some(1));
    assert_eq!(facts.name, "the rest");
}

#[test]
fn name_of_an_advisory_row_is_its_crate_at_its_version() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let row = Concern::Yanked {
        name: String::from("gizmo"),
        version: version("2.7.5"),
    }
    .row();

    // Act
    let name = FactFinder::new(&context).name(&row);

    // Assert
    assert_eq!(name, "gizmo 2.7.5");
}

#[test]
fn name_of_the_toolchain_says_so() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = breaking("rust", "1.99.1", "1.100.0");
    unit.kind = UnitKind::Toolchain;

    // Act
    let name = FactFinder::new(&context).name(&unit);

    // Assert
    assert_eq!(name, "the toolchain");
}

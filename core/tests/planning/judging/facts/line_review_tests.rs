// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::planning::choice::line_stats::LineStats;
use ripe4rust_core::planning::judging::facts::line_review::LineReview;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::judging::line_standing::LineStanding;
use ripe4rust_core::values::line::Line;

#[test]
fn behind_counts_the_candidate_lines_past_the_one_in_use() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let behind = LineReview::new(&context).behind(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(behind, 1);
}

#[test]
fn in_use_of_a_unit_is_the_line_of_its_version() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let in_use = LineReview::new(&context).in_use(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(
        in_use,
        Some(LineStanding {
            stats: LineStats {
                line: Line {
                    major: 1,
                    minor: None
                },
                first_release: Some(date("2024-01-10")),
                last_release: Some(date("2026-10-01")),
                releases: 3,
            },
            young: false,
            active: true,
        })
    );
}

#[test]
fn new_line_of_a_compatible_unit_is_none() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let new_line = LineReview::new(&context).new_line(&compatible(
        "widget",
        vec![moved("widget", "1.4.0", "1.4.1")],
    ));

    // Assert
    assert_eq!(new_line, None);
}

#[test]
fn new_line_of_a_group_is_young_where_any_member_s_is() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut group = breaking("widget", "1.4.0", "2.1.0");
    group.members.push(String::from("gadget"));
    group.moves.push(moved("gadget", "0.13.1", "0.14.0"));

    // Act
    let new_line = LineReview::new(&context).new_line(&group);

    // Assert
    assert_eq!(
        new_line.map(|standing| (standing.stats.line, standing.young)),
        Some((
            Line {
                major: 2,
                minor: None
            },
            true
        ))
    );
}

#[test]
fn new_line_of_a_settled_target_is_not_young() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let new_line = LineReview::new(&context).new_line(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(new_line.map(|standing| standing.young), Some(false));
}

#[test]
fn newer_active_of_a_unit_without_history_is_false() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let active = LineReview::new(&context).newer_active(&breaking("unknown", "1.0.0", "2.0.0"));

    // Assert
    assert!(!active);
}

#[test]
fn newer_active_of_an_active_newer_line_is_true() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let active = LineReview::new(&context).newer_active(&breaking("gadget", "0.13.1", "0.14.0"));

    // Assert
    assert!(active);
}

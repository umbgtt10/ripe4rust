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
use ripe4rust_core::planning::choice::line_stats::LineStats;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::line::Line;

fn toolchain_unit() -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: Some(version("1.99.1")),
        to: Some(version("1.100.0")),
        moves: Vec::new(),
        held: Vec::new(),
    }
}

#[test]
fn histories_are_the_crates_read() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let read = context.histories().get("gizmo").is_some();

    // Assert
    assert!(read);
}

#[test]
fn history_of_a_crate_s_unit_is_its_first_member_s() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let history = context.history_of(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(
        history.map(|history| history
            .line_stats(Line {
                major: 2,
                minor: None
            })
            .releases),
        Some(3)
    );
}

#[test]
fn history_of_the_rest_is_none() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut rest = compatible("libc", vec![moved("libc", "0.2.177", "0.2.178")]);
    rest.members = Vec::new();

    // Act
    let history = context.history_of(&rest);

    // Assert
    assert!(history.is_none());
}

#[test]
fn history_of_the_toolchain_is_its_releases() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let history = context.history_of(&toolchain_unit());

    // Assert
    assert_eq!(
        history.map(|history| history.candidates(&version("1.99.1"))),
        Some(vec![
            version("1.99.0"),
            version("1.99.1"),
            version("1.100.0")
        ])
    );
}

#[test]
fn maturity_of_a_crate_counts_its_releases() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let one_old_release = LineStats {
        line: Line {
            major: 1,
            minor: None,
        },
        first_release: Some(date("2020-01-01")),
        last_release: Some(date("2020-01-01")),
        releases: 1,
    };

    // Act
    let young = context.maturity().is_young(&one_old_release);

    // Assert
    assert!(young);
}

#[test]
fn maturity_of_the_toolchain_judges_youth_by_days_alone() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let one_release_today = LineStats {
        line: Line {
            major: 1,
            minor: Some(100),
        },
        first_release: Some(surveyed.today),
        last_release: Some(surveyed.today),
        releases: 1,
    };

    // Act
    let young = context
        .maturity_of(&toolchain_unit())
        .is_young(&one_release_today);

    // Assert
    assert!(!young);
}

#[test]
fn surveyed_is_the_survey_given() {
    // Arrange
    let (surveyed, toolchain) = (surveyed(), toolchain(&surveyed()));
    let histories = histories(&surveyed);
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let given = context.surveyed();

    // Assert
    assert_eq!(given, &surveyed);
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::releases::release;
use crate::support::releases::version;
use crate::support::surveyed::at;
use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::git;
use crate::support::units::moved;
use ripe4rust_core::planning::judging::facts::pin_review::PinReview;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::planning::surveyed::Surveyed;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::requirement::Requirement;

fn pin(name: &str, hold: Hold) -> Pin {
    Pin {
        name: String::from(name),
        reason: String::from("a migration"),
        review: Some(date("2027-01-31")),
        until: None,
        hold,
        at: at("Cargo.toml", 41),
    }
}

fn pinned(pins: Vec<Pin>) -> Surveyed {
    let mut surveyed = surveyed();
    surveyed.policy.pins = pins;
    surveyed
}

fn rust(from: &str, to: &str) -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Toolchain, "rust"),
        kind: UnitKind::Toolchain,
        name: Some(String::from("rust")),
        members: vec![String::from("rust")],
        from: Some(version(from)),
        to: Some(version(to)),
        moves: Vec::new(),
        held: Vec::new(),
        git: None,
    }
}

#[test]
fn fully_held_of_a_breaking_unit_without_pins_is_false() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let held = PinReview::new(&context).fully_held(&breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert!(!held);
}

#[test]
fn fully_held_of_a_compatible_unit_with_nothing_left_is_true() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = compatible("widget", Vec::new());
    unit.held = vec![moved("gizmo", "2.7.5", "2.7.6")];

    // Act
    let held = PinReview::new(&context).fully_held(&unit, &[]);

    // Assert
    assert!(held);
}

#[test]
fn pins_of_a_breaking_unit_are_its_own_on_any_hold() {
    // Arrange
    let surveyed = pinned(vec![pin("widget", Hold::Line), pin("gadget", Hold::All)]);
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let pins = PinReview::new(&context).pins(&breaking("widget", "1.4.0", "2.1.0"));

    // Assert
    assert_eq!(
        pins.iter()
            .map(|standing| standing.pin.name.clone())
            .collect::<Vec<_>>(),
        vec![String::from("widget")]
    );
}

#[test]
fn pins_of_a_compatible_unit_are_the_all_pins_on_its_held_moves() {
    // Arrange
    let surveyed = pinned(vec![pin("gizmo", Hold::All), pin("widget", Hold::Line)]);
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let mut unit = compatible("widget", vec![moved("widget", "1.4.0", "1.4.1")]);
    unit.held = vec![moved("gizmo", "2.7.5", "2.7.6")];

    // Act
    let pins = PinReview::new(&context).pins(&unit);

    // Assert
    assert_eq!(
        pins.iter()
            .map(|standing| standing.pin.name.clone())
            .collect::<Vec<_>>(),
        vec![String::from("gizmo")]
    );
}

#[test]
fn pins_of_a_git_unit_are_those_on_any_of_its_crates_whatever_they_hold() {
    // Arrange
    let surveyed = pinned(vec![pin("gitmacros", Hold::Line), pin("widget", Hold::All)]);
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let unit = git("gitcore", &["gitcore", "gitmacros"], Vec::new());
    let review = PinReview::new(&context);

    // Act
    let pins = review.pins(&unit);

    // Assert
    assert_eq!(
        (
            pins.iter()
                .map(|standing| standing.pin.name.clone())
                .collect::<Vec<String>>(),
            review.fully_held(&unit, &pins)
        ),
        (vec![String::from("gitmacros")], true)
    );
}

#[test]
fn pins_of_the_toolchain_on_its_line_let_a_patch_release_through() {
    // Arrange
    let surveyed = pinned(vec![pin("rust", Hold::Line)]);
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let review = PinReview::new(&context);

    // Act & Assert
    assert_eq!(review.pins(&rust("1.99.0", "1.99.1")).len(), 0);
    assert_eq!(review.pins(&rust("1.99.1", "1.100.0")).len(), 1);
}

#[test]
fn released_of_a_met_condition_names_the_release() {
    // Arrange
    let mut surveyed = surveyed();
    surveyed.releases.push((
        String::from("widget-tools"),
        vec![release("0.32.0", "2026-11-01")],
    ));
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let waiting = Pin {
        until: Some(ReleaseCondition::Release {
            name: String::from("widget-tools"),
            requirement: Requirement::parse(">=0.32").expect("a requirement"),
        }),
        ..pin("widget", Hold::Line)
    };

    // Act
    let released = PinReview::new(&context).released(&waiting);

    // Assert
    assert_eq!(released, Some(String::from("widget-tools 0.32.0 released")));
}

#[test]
fn released_of_a_pin_still_holding_is_none() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);

    // Act
    let released = PinReview::new(&context).released(&pin("widget", Hold::Line));

    // Assert
    assert_eq!(released, None);
}

#[test]
fn released_of_a_review_date_reached_says_so() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let due = Pin {
        review: Some(date("2026-11-26")),
        ..pin("widget", Hold::Line)
    };

    // Act
    let released = PinReview::new(&context).released(&due);

    // Assert
    assert_eq!(released, Some(String::from("review 2026-11-26 reached")));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recorded::plan;
use crate::support::recorded::plan_with_git;
use crate::support::releases::version;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::round::holding::held_crate::HeldCrate;
use ripe4rust_core::planning::round::holding::held_crates::HeldCrates;
use ripe4rust_core::planning::round::target_finder::TargetFinder;

fn held(name: &str, hold: Hold) -> HeldCrate {
    HeldCrate {
        name: String::from(name),
        hold,
    }
}

fn of(id: &str) -> Vec<HeldCrate> {
    let plan = plan();
    HeldCrates.of(&TargetFinder::new(&plan).find(id).expect("a unit"))
}

#[test]
fn of_a_breaking_unit_holds_its_crate_on_its_line() {
    // Arrange & Act
    let crates = of("breaking:gadget");

    // Assert
    assert_eq!(crates, vec![held("gadget", Hold::Line)]);
}

#[test]
fn of_a_compatible_unit_holds_every_crate_it_moves_whole() {
    // Arrange & Act
    let crates = (of("compatible:widget"), of("rest"));

    // Assert
    assert_eq!(
        crates,
        (
            vec![held("widget", Hold::All), held("gizmo", Hold::All)],
            vec![held("libc", Hold::All)]
        )
    );
}

#[test]
fn of_a_git_unit_holds_its_crate_whole() {
    // Arrange
    let plan = plan_with_git();

    // Act
    let crates = HeldCrates.of(&TargetFinder::new(&plan)
        .find("git:gitcore")
        .expect("a unit"));

    // Assert
    assert_eq!(crates, vec![held("gitcore", Hold::All)]);
}

#[test]
fn of_a_split_move_holds_its_crate_whole() {
    // Arrange & Act
    let crates = of("compatible:widget/gizmo");

    // Assert
    assert_eq!(crates, vec![held("gizmo", Hold::All)]);
}

#[test]
fn of_the_toolchain_holds_a_new_minor_on_its_line_and_a_patch_whole() {
    // Arrange
    let mut plan = plan();
    let minor = HeldCrates.of(&TargetFinder::new(&plan)
        .find("toolchain:rust")
        .expect("a unit"));
    plan.units[1].to = Some(version("1.99.2"));

    // Act
    let patch = HeldCrates.of(&TargetFinder::new(&plan)
        .find("toolchain:rust")
        .expect("a unit"));

    // Assert
    assert_eq!(
        (minor, patch),
        (
            vec![held("rust", Hold::Line)],
            vec![held("rust", Hold::All)]
        )
    );
}

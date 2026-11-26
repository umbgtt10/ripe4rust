// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use crate::support::releases::version;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::planning::writing::answer::Answer;
use ripe4rust_core::planning::writing::option_writer::OptionWriter;
use ripe4rust_core::planning::writing::question_option::QuestionOption;
use ripe4rust_core::unit_kind::UnitKind;

fn concerns() -> Vec<Concern> {
    vec![
        Concern::Advisory(Advisory {
            id: String::from("RUSTSEC-2026-0190"),
            name: String::from("widget"),
            version: version("1.0.0"),
            patched: Vec::new(),
            unaffected: Vec::new(),
        }),
        Concern::Yanked {
            name: String::from("spin"),
            version: version("0.10.0"),
        },
    ]
}

fn means_and_then(answer: Answer, facts: &UnitFacts) -> (String, String) {
    let id = UnitId::of(UnitKind::Breaking, "widget");
    let concerns = concerns();
    let option = OptionWriter::new(&id, facts, &concerns).option(answer);
    (option.means, option.then)
}

#[test]
fn option_of_accept_adds_every_concern_to_the_ignore_list() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::Accept, &facts(UnitKind::Breaking));

    // Assert
    assert_eq!(means, "accept it in `deny.toml`, with the person's reason");
    assert_eq!(
        then,
        "add `{ id = \"RUSTSEC-2026-0190\", reason = \"<the person's words>\" }` and `{ crate = \"spin@0.10.0\", reason = \"<the person's words>\" }` to `[advisories] ignore` in `deny.toml`, commit it, and plan again"
    );
}

#[test]
fn option_of_declare_gates_says_where_they_go() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::DeclareGates, &facts(UnitKind::Compatible));

    // Assert
    assert_eq!(means, "declare the commands that validate a change");
    assert_eq!(
        then,
        "add `gates = [<the person's commands>]` to `[workspace.metadata.ripe4rust]` in `Cargo.toml`, commit it, and plan again"
    );
}

#[test]
fn option_of_keep_pinned_sets_a_new_review_date() {
    // Arrange
    let pinned = UnitFacts {
        pins: vec![pinned(Some("review 2026-11-01 reached"))],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let (means, then) = means_and_then(Answer::KeepPinned, &pinned);

    // Assert
    assert_eq!(means, "keep it, with a new review date");
    assert_eq!(
        then,
        "set the `review` of the pin on `widget` at Cargo.toml:41 to the person's date, commit it, and plan again"
    );
}

#[test]
fn option_of_migrate_now_beyond_the_gates_validates_on_the_target() {
    // Arrange
    let reaching = UnitFacts {
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let (means, _) = means_and_then(Answer::MigrateNow, &reaching);

    // Assert
    assert_eq!(means, "apply it now and validate on thumbv7em-none-eabihf");
}

#[test]
fn option_of_migrate_now_names_the_work_list() {
    // Arrange & Act
    let option = OptionWriter::new(
        &UnitId::of(UnitKind::Breaking, "widget"),
        &facts(UnitKind::Breaking),
        &[],
    )
    .option(Answer::MigrateNow);

    // Assert
    assert_eq!(
        option,
        QuestionOption {
            answer: Answer::MigrateNow,
            means: String::from("apply it now"),
            then: String::from(
                "apply it as a ripe unit; the footprint's 42 locations are the work list"
            ),
        }
    );
}

#[test]
fn option_of_migrate_now_with_a_footprint_that_is_a_floor_says_so() {
    // Arrange
    let floor = UnitFacts {
        footprint: Some(Footprint {
            lines: 3,
            files: 1,
            complete: false,
        }),
        ..facts(UnitKind::Breaking)
    };

    // Act
    let (_, then) = means_and_then(Answer::MigrateNow, &floor);

    // Assert
    assert_eq!(
        then,
        "apply it as a ripe unit; the footprint's 3 locations are a floor of the work list, as code is included the parse cannot follow: the build's errors name the rest"
    );
}

#[test]
fn option_of_migrate_now_with_an_unknown_footprint_says_the_build_is_the_work_list() {
    // Arrange
    let unknown = UnitFacts {
        footprint: None,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let (_, then) = means_and_then(Answer::MigrateNow, &unknown);

    // Assert
    assert_eq!(
        then,
        "apply it as a ripe unit; the footprint is not known, so the build's errors are the work list"
    );
}

#[test]
fn option_of_plan_it_holds_with_a_review_date() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::PlanIt, &facts(UnitKind::Breaking));

    // Assert
    assert_eq!(means, "hold it until a planned migration");
    assert_eq!(
        then,
        "cargo ripe4rust hold breaking:widget --plan plan.json --because \"<the person's words>\" --review <date>"
    );
}

#[test]
fn option_of_replace_names_the_crate() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::Replace, &facts(UnitKind::Advisory));

    // Assert
    assert_eq!(means, "replace the crate");
    assert_eq!(
        then,
        "replace `widget` and `spin` as the person decides, commit it, and plan again"
    );
}

#[test]
fn option_of_take_it_applies_it_as_ripe() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::TakeIt, &facts(UnitKind::Compatible));

    // Assert
    assert_eq!(means, "apply it now");
    assert_eq!(then, "apply it as a ripe unit");
}

#[test]
fn option_of_unpin_removes_the_pin() {
    // Arrange
    let pinned = UnitFacts {
        pins: vec![pinned(None)],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let (means, then) = means_and_then(Answer::Unpin, &pinned);

    // Assert
    assert_eq!(means, "remove the pin and take the upgrade");
    assert_eq!(
        then,
        "remove the pin on `widget` at Cargo.toml:41, commit it, and plan again"
    );
}

#[test]
fn option_of_wait_holds_without_a_review_date() {
    // Arrange & Act
    let (means, then) = means_and_then(Answer::Wait, &facts(UnitKind::Breaking));

    // Assert
    assert_eq!(means, "not now");
    assert_eq!(
        then,
        "cargo ripe4rust hold breaking:widget --plan plan.json --because \"<the person's words>\""
    );
}

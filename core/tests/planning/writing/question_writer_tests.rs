// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::facts::facts;
use crate::support::facts::pinned;
use crate::support::facts::standing;
use crate::support::releases::version;
use crate::support::units::breaking;
use crate::support::units::compatible;
use crate::support::units::moved;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::planning::judging::judge::Judge;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::planning::units::concern::Concern;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::writing::answer::Answer;
use ripe4rust_core::planning::writing::question::Question;
use ripe4rust_core::planning::writing::question_writer::QuestionWriter;
use ripe4rust_core::unit_kind::UnitKind;

fn advisory() -> Concern {
    Concern::Advisory(Advisory {
        id: String::from("RUSTSEC-2026-0190"),
        name: String::from("widget"),
        version: version("1.0.0"),
        patched: Vec::new(),
        unaffected: Vec::new(),
    })
}

fn answers(question: Option<Question>) -> Option<(Answer, Vec<Answer>)> {
    question.map(|question| {
        (
            question.recommendation,
            question
                .options
                .iter()
                .map(|option| option.answer)
                .collect(),
        )
    })
}

fn asked(unit: &Unit, facts: &UnitFacts, concerns: &[Concern]) -> Option<Question> {
    QuestionWriter.question(unit, &Judge.judge(facts), facts, concerns)
}

fn widget() -> Unit {
    compatible("widget", vec![moved("widget", "1.0.0", "1.0.1")])
}

#[test]
fn question_of_a_breaking_unit_beyond_the_gates_recommends_the_verdict() {
    // Arrange
    let unit = breaking("widget", "0.13.1", "0.14.0");
    let reaching = UnitFacts {
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let question = asked(&unit, &reaching, &[]).expect("a question");

    // Assert
    assert_eq!(question.title, "Move widget to 0.14.0?");
    assert_eq!(
        question.why,
        "it reaches thumbv7em-none-eabihf, which the gates cannot validate"
    );
    assert_eq!(question.recommendation, Answer::PlanIt);
    assert_eq!(
        question
            .options
            .iter()
            .map(|option| (option.answer, option.means.clone()))
            .collect::<Vec<_>>(),
        vec![
            (
                Answer::MigrateNow,
                String::from("apply it now and validate on thumbv7em-none-eabihf")
            ),
            (
                Answer::PlanIt,
                String::from("hold it until a planned migration")
            ),
            (Answer::Wait, String::from("not now")),
        ]
    );
}

#[test]
fn question_of_a_compatible_unit_beyond_the_gates_offers_take_it_or_wait() {
    // Arrange
    let reaching = UnitFacts {
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let answers = answers(asked(&widget(), &reaching, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::TakeIt, vec![Answer::TakeIt, Answer::Wait]))
    );
}

#[test]
fn question_of_a_due_pin_offers_unpin_or_keep_it() {
    // Arrange
    let due = UnitFacts {
        pins: vec![pinned(Some("review 2026-11-01 reached"))],
        fully_held: true,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let answers = answers(asked(&breaking("widget", "1.0.0", "2.0.0"), &due, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::Unpin, vec![Answer::Unpin, Answer::KeepPinned]))
    );
}

#[test]
fn question_of_a_pinned_unit_overripe_offers_unpin_or_keep_it() {
    // Arrange
    let overripe = UnitFacts {
        pins: vec![pinned(None)],
        fully_held: true,
        lines_behind: 2,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let answers = answers(asked(&breaking("widget", "1.0.0", "3.0.0"), &overripe, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::Unpin, vec![Answer::Unpin, Answer::KeepPinned]))
    );
}

#[test]
fn question_of_a_ripe_unit_is_none() {
    // Arrange & Act
    let question = asked(&widget(), &facts(UnitKind::Compatible), &[]);

    // Assert
    assert_eq!(question, None);
}

#[test]
fn question_of_a_young_breaking_line_overripe_recommends_migrating() {
    // Arrange
    let overripe = UnitFacts {
        new_line: Some(standing(3, true, true)),
        lines_behind: 2,
        ..facts(UnitKind::Breaking)
    };

    // Act
    let answers = answers(asked(&breaking("widget", "1.0.0", "3.0.0"), &overripe, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((
            Answer::MigrateNow,
            vec![Answer::MigrateNow, Answer::PlanIt, Answer::Wait]
        ))
    );
}

#[test]
fn question_of_a_young_toolchain_overripe_recommends_taking_it() {
    // Arrange
    let overripe = UnitFacts {
        new_line: Some(standing(1, true, true)),
        lines_behind: 3,
        ..facts(UnitKind::Toolchain)
    };

    // Act
    let answers = answers(asked(&widget(), &overripe, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::TakeIt, vec![Answer::TakeIt, Answer::Wait]))
    );
}

#[test]
fn question_of_an_advisory_row_offers_replace_or_accept() {
    // Arrange
    let row = advisory().row();
    let facts = UnitFacts {
        name: String::from("widget 1.0.0"),
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        ..facts(UnitKind::Advisory)
    };

    // Act
    let question = asked(&row, &facts, &[advisory()]).expect("a question");

    // Assert
    assert_eq!(question.title, "Resolve RUSTSEC-2026-0190 in widget 1.0.0?");
    assert_eq!(question.recommendation, Answer::Replace);
    assert_eq!(
        question
            .options
            .iter()
            .map(|option| option.answer)
            .collect::<Vec<_>>(),
        vec![Answer::Replace, Answer::Accept]
    );
}

#[test]
fn question_of_an_ungated_unit_offers_to_declare_the_gates() {
    // Arrange
    let ungated = UnitFacts {
        gates: 0,
        ..facts(UnitKind::Compatible)
    };

    // Act
    let answers = answers(asked(&widget(), &ungated, &[]));

    // Assert
    assert_eq!(
        answers,
        Some((
            Answer::DeclareGates,
            vec![Answer::DeclareGates, Answer::TakeIt, Answer::Wait]
        ))
    );
}

#[test]
fn question_of_an_urgent_breaking_unit_offers_migrate_now_or_accept() {
    // Arrange
    let urgent = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        ..facts(UnitKind::Breaking)
    };

    // Act
    let answers = answers(asked(
        &breaking("widget", "1.0.0", "2.0.0"),
        &urgent,
        &[advisory()],
    ));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::MigrateNow, vec![Answer::MigrateNow, Answer::Accept]))
    );
}

#[test]
fn question_of_an_urgent_ungated_unit_offers_no_hold() {
    // Arrange
    let urgent = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        gates: 0,
        ..facts(UnitKind::Compatible)
    };

    // Act
    let answers = answers(asked(&widget(), &urgent, &[advisory()]));

    // Assert
    assert_eq!(
        answers,
        Some((
            Answer::DeclareGates,
            vec![Answer::DeclareGates, Answer::TakeIt, Answer::Accept]
        ))
    );
}

#[test]
fn question_of_an_urgent_unit_beyond_the_gates_offers_no_hold() {
    // Arrange
    let urgent = UnitFacts {
        concerns: vec![String::from("RUSTSEC-2026-0190")],
        reaches: vec![String::from("thumbv7em-none-eabihf")],
        ..facts(UnitKind::Compatible)
    };

    // Act
    let answers = answers(asked(&widget(), &urgent, &[advisory()]));

    // Assert
    assert_eq!(
        answers,
        Some((Answer::TakeIt, vec![Answer::TakeIt, Answer::Accept]))
    );
}

#[test]
fn question_of_the_rest_moves_it_without_a_version() {
    // Arrange
    let mut rest = widget();
    rest.to = None;
    let ungated = UnitFacts {
        name: String::from("the rest"),
        gates: 0,
        ..facts(UnitKind::Compatible)
    };

    // Act
    let question = asked(&rest, &ungated, &[]).expect("a question");

    // Assert
    assert_eq!(question.title, "Move the rest?");
}

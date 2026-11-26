// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::histories;
use crate::support::surveyed::surveyed;
use crate::support::surveyed::toolchain;
use crate::support::units::breaking;
use ripe4rust_core::outcome::Outcome;
use ripe4rust_core::planning::judging::facts::plan_context::PlanContext;
use ripe4rust_core::planning::unit_writer::UnitWriter;
use ripe4rust_core::planning::writing::answer::Answer;
use ripe4rust_core::planning::writing::step_writer::StepWriter;

#[test]
fn planned_of_a_breaking_unit_in_a_published_api_asks_it_with_its_steps() {
    // Arrange
    let surveyed = surveyed();
    let (histories, toolchain) = (histories(&surveyed), toolchain(&surveyed));
    let context = PlanContext::new(&surveyed, &histories, &toolchain);
    let steps = StepWriter::new(surveyed.policy.gates.clone(), Vec::new());

    // Act
    let planned =
        UnitWriter::new(&context, &steps).planned(breaking("widget", "1.4.0", "2.1.0"), &[]);

    // Assert
    assert_eq!(planned.decision.outcome, Outcome::Ask);
    assert_eq!(planned.decision.rule, Some(8));
    assert_eq!(planned.summary, "Migrate widget to 2.1.0");
    assert_eq!(planned.steps.map(|steps| steps.apply.len()), Some(3));
    assert_eq!(
        planned.question.map(|question| question.recommendation),
        Some(Answer::PlanIt)
    );
    assert_eq!(planned.evidence.footprint.len(), 3);
}

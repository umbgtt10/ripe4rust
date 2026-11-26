// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::fact_finder::FactFinder;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::judging::judge::Judge;
use crate::planning::planned_unit::PlannedUnit;
use crate::planning::units::concern::Concern;
use crate::planning::units::unit::Unit;
use crate::planning::writing::edit_finder::EditFinder;
use crate::planning::writing::evidence_writer::EvidenceWriter;
use crate::planning::writing::question_writer::QuestionWriter;
use crate::planning::writing::step_writer::StepWriter;
use crate::planning::writing::summary_writer::SummaryWriter;

/// One row of the plan: the unit judged, its steps, its question, its
/// evidence and its summary.
pub struct UnitWriter<'a> {
    context: &'a PlanContext<'a>,
    steps: &'a StepWriter,
}

impl<'a> UnitWriter<'a> {
    pub fn new(context: &'a PlanContext<'a>, steps: &'a StepWriter) -> Self {
        Self { context, steps }
    }

    pub fn planned(&self, unit: Unit, concerns: &[Concern]) -> PlannedUnit {
        let facts = FactFinder::new(self.context).facts(&unit, concerns);
        let decision = Judge.judge(&facts);
        let edits = EditFinder::new(self.context).edits(&unit);
        PlannedUnit {
            steps: self.steps.steps(&unit, &edits),
            question: QuestionWriter.question(&unit, &decision, &facts, concerns),
            evidence: EvidenceWriter::new(self.context).evidence(&unit, concerns),
            summary: SummaryWriter.summary(&unit, concerns),
            decision,
            unit,
        }
    }
}

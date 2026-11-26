// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::named_nowhere::NamedNowhere;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::verdict::Verdict;
use crate::planning::judging::wording::Wording;
use crate::planning::units::concern::Concern;
use crate::planning::units::own_move::OwnMove;
use crate::planning::units::unit::Unit;
use crate::planning::writing::answer::Answer;
use crate::planning::writing::option_writer::OptionWriter;
use crate::planning::writing::question::Question;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

/// The question of a unit that asks, titled with the move it makes -- the
/// crates it pulls in where its own crate stays: an advisory row offers to replace the
/// crate or accept it; an urgent unit is never held, so it offers to take it
/// or accept it; a due or overripe pin offers to unpin or keep it; no gates
/// offers to declare them; a breaking unit offers migrate now, plan it or
/// wait, its verdict recommended; anything else, take it or wait. A breaking
/// unit whose crate is named nowhere also offers to remove it.
pub struct QuestionWriter;

impl QuestionWriter {
    pub fn question(
        &self,
        unit: &Unit,
        decision: &Decision,
        facts: &UnitFacts,
        concerns: &[Concern],
    ) -> Option<Question> {
        if decision.outcome != Outcome::Ask {
            return None;
        }
        let (recommendation, mut answers) = Self::answers(decision, facts);
        if NamedNowhere.fits(facts) {
            answers.push(Answer::Remove);
        }
        let options = OptionWriter::new(&unit.id, facts, concerns);
        let title = match (unit.kind, &unit.to) {
            (UnitKind::Advisory, _) => format!(
                "Resolve {} in {}?",
                Wording.list(&facts.concerns),
                facts.name
            ),
            (_, Some(to)) => format!("Move {} to {to}?", facts.name),
            (_, None) if OwnMove.stays(unit) => {
                let pulled = unit
                    .moves
                    .iter()
                    .filter(|lock_move| !unit.members.contains(&lock_move.name))
                    .count();
                format!(
                    "Move the {} {} pulls in?",
                    Wording.count(u32::try_from(pulled).unwrap_or(u32::MAX), "crate"),
                    facts.name
                )
            }
            (_, None) => format!("Move {}?", facts.name),
        };
        Some(Question {
            title,
            why: decision.why.clone().unwrap_or_default(),
            recommendation,
            options: answers
                .into_iter()
                .map(|answer| options.option(answer))
                .collect(),
        })
    }

    fn answers(decision: &Decision, facts: &UnitFacts) -> (Answer, Vec<Answer>) {
        let breaking = facts.kind == UnitKind::Breaking;
        let now = if breaking {
            Answer::MigrateNow
        } else {
            Answer::TakeIt
        };
        let pin = matches!(decision.suggestion, Some(Suggestion::RevisitThePin))
            || (decision.suggestion == Some(Suggestion::Overripe) && decision.rule == Some(4));
        match (facts.kind, decision.suggestion, decision.urgent) {
            (UnitKind::Advisory, ..) => (Answer::Replace, vec![Answer::Replace, Answer::Accept]),
            (_, Some(Suggestion::DeclareTheGates), true) => (
                Answer::DeclareGates,
                vec![Answer::DeclareGates, now, Answer::Accept],
            ),
            (_, Some(Suggestion::DeclareTheGates), false) => (
                Answer::DeclareGates,
                vec![Answer::DeclareGates, now, Answer::Wait],
            ),
            (_, _, true) => (now, vec![now, Answer::Accept]),
            _ if pin => (Answer::Unpin, vec![Answer::Unpin, Answer::KeepPinned]),
            (_, Some(Suggestion::Overripe), _) if breaking => (
                Answer::MigrateNow,
                vec![Answer::MigrateNow, Answer::PlanIt, Answer::Wait],
            ),
            _ if breaking => {
                let verdict = match Verdict.call(facts).suggestion {
                    Suggestion::MigrateNow => Answer::MigrateNow,
                    Suggestion::PlanIt => Answer::PlanIt,
                    _ => Answer::Wait,
                };
                (
                    verdict,
                    vec![Answer::MigrateNow, Answer::PlanIt, Answer::Wait],
                )
            }
            _ => (now, vec![now, Answer::Wait]),
        }
    }
}

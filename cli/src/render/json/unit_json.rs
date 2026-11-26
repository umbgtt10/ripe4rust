// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::unit_document::UnitDocument;
use crate::render::json::evidence_json::EvidenceJson;
use crate::render::json::move_json::MoveJson;
use crate::render::json::question_json::QuestionJson;
use crate::render::json::steps_json::StepsJson;
use ripe4rust_core::planning::planned_unit::PlannedUnit;

/// One row of the plan: the unit -- a git unit's revs as its `from` and
/// `to`, beside its `source` --, its decision, its steps, its question and its
/// evidence.
pub struct UnitJson;

impl UnitJson {
    pub fn document<'a>(&self, planned: &'a PlannedUnit) -> UnitDocument<'a> {
        let unit = &planned.unit;
        let decision = &planned.decision;
        UnitDocument {
            id: unit.id.to_string(),
            kind: unit.kind.label(),
            name: unit.name.as_deref(),
            members: &unit.members,
            from: unit.git.as_ref().map_or_else(
                || unit.from.as_ref().map(ToString::to_string),
                |git| Some(git.from.clone()),
            ),
            to: unit.git.as_ref().map_or_else(
                || unit.to.as_ref().map(ToString::to_string),
                |git| Some(git.to.clone()),
            ),
            source: unit.git.as_ref().map(|git| git.source.as_str()),
            moves: unit
                .moves
                .iter()
                .map(|lock_move| MoveJson.document(lock_move))
                .collect(),
            outcome: decision.outcome.label(),
            urgent: decision.urgent,
            rule: decision.rule,
            suggestion: decision.suggestion.map(|suggestion| suggestion.label()),
            reasons: &decision.reasons,
            summary: &planned.summary,
            steps: planned
                .steps
                .as_ref()
                .map(|steps| StepsJson.document(steps)),
            question: planned
                .question
                .as_ref()
                .map(|question| QuestionJson.document(question)),
            evidence: EvidenceJson.document(&planned.evidence),
        }
    }
}

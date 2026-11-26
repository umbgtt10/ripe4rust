// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::evidence_json::EvidenceJson;
use crate::render::json::move_json::MoveJson;
use crate::render::json::question_json::QuestionJson;
use crate::render::json::steps_json::StepsJson;
use ripe4rust_core::planning::planned_unit::PlannedUnit;
use serde_json::Value;
use serde_json::json;

/// One row of the plan: the unit, its decision, its steps, its question and
/// its evidence.
pub struct UnitJson;

impl UnitJson {
    pub fn value(&self, planned: &PlannedUnit) -> Value {
        let unit = &planned.unit;
        let decision = &planned.decision;
        json!({
            "id": unit.id.to_string(),
            "kind": unit.kind.label(),
            "crate": unit.name,
            "members": unit.members,
            "from": unit.from.as_ref().map(ToString::to_string),
            "to": unit.to.as_ref().map(ToString::to_string),
            "moves": unit.moves.iter().map(|lock_move| MoveJson.value(lock_move)).collect::<Vec<Value>>(),
            "outcome": decision.outcome.label(),
            "urgent": decision.urgent,
            "rule": decision.rule,
            "suggestion": decision.suggestion.map(|suggestion| suggestion.label()),
            "reasons": decision.reasons,
            "summary": planned.summary,
            "steps": planned.steps.as_ref().map(|steps| StepsJson.value(steps)),
            "question": planned.question.as_ref().map(|question| QuestionJson.value(question)),
            "evidence": EvidenceJson.value(&planned.evidence),
        })
    }
}

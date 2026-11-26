// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::evidence_document::EvidenceDocument;
use crate::render::json::documents::move_document::MoveDocument;
use crate::render::json::documents::question_document::QuestionDocument;
use crate::render::json::documents::steps_document::StepsDocument;
use serde::Serialize;

/// One row of the plan, in the design's order: `members` beside `crate`,
/// `source` after `to`, `reasons` before `summary`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UnitDocument<'a> {
    pub id: String,
    pub kind: &'a str,
    #[serde(rename = "crate")]
    pub name: Option<&'a str>,
    pub members: &'a [String],
    pub from: Option<String>,
    pub to: Option<String>,
    pub source: Option<&'a str>,
    pub moves: Vec<MoveDocument<'a>>,
    pub outcome: &'a str,
    pub urgent: bool,
    pub rule: Option<u8>,
    pub suggestion: Option<&'a str>,
    pub reasons: &'a [String],
    pub summary: &'a str,
    pub steps: Option<StepsDocument<'a>>,
    pub question: Option<QuestionDocument<'a>>,
    pub evidence: EvidenceDocument<'a>,
}

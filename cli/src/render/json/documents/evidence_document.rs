// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::dependent_document::DependentDocument;
use crate::render::json::documents::line_document::LineDocument;
use crate::render::json::documents::move_document::MoveDocument;
use serde::Serialize;

/// What a unit's decision read, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EvidenceDocument<'a> {
    pub line_in_use: Option<LineDocument>,
    pub rust_version: Option<String>,
    pub advisories: &'a [String],
    pub yanked_in_use: bool,
    pub footprint: Vec<String>,
    pub held_back: &'a [String],
    pub held: Vec<MoveDocument<'a>>,
    pub downgrades: Vec<MoveDocument<'a>>,
    pub unfollowed_includes: Vec<String>,
    pub held_by: Vec<DependentDocument<'a>>,
}

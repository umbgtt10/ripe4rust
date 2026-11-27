// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::dependent_document::DependentDocument;
use crate::render::json::documents::evidence_document::EvidenceDocument;
use crate::render::json::documents::line_document::LineDocument;
use crate::render::json::move_json::MoveJson;
use ripe4rust_core::planning::writing::evidence::Evidence;

/// What a unit's decision read.
pub struct EvidenceJson;

impl EvidenceJson {
    pub fn document<'a>(&self, evidence: &'a Evidence) -> EvidenceDocument<'a> {
        EvidenceDocument {
            line_in_use: evidence.line_in_use.map(|line| LineDocument {
                line: line.line.to_string(),
                first_release: line.first_release.map(|date| date.to_string()),
                last_release: line.last_release.map(|date| date.to_string()),
                releases: line.releases,
            }),
            rust_version: evidence.rust_version.as_ref().map(ToString::to_string),
            advisories: &evidence.advisories,
            yanked_in_use: evidence.yanked_in_use,
            footprint: evidence.footprint.iter().map(ToString::to_string).collect(),
            held_back: &evidence.held_back,
            held: evidence
                .held
                .iter()
                .map(|held| MoveJson.document(held))
                .collect(),
            downgrades: evidence
                .downgrades
                .iter()
                .map(|downgrade| MoveJson.document(downgrade))
                .collect(),
            unfollowed_includes: evidence
                .unfollowed_includes
                .iter()
                .map(ToString::to_string)
                .collect(),
            held_by: evidence
                .held_by
                .iter()
                .map(|dependent| DependentDocument {
                    name: &dependent.name,
                    version: dependent.version.to_string(),
                    requires: &dependent.requires,
                    requirement: dependent.requirement.to_string(),
                })
                .collect(),
        }
    }
}

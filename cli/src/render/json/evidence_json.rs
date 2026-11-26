// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::move_json::MoveJson;
use ripe4rust_core::planning::writing::evidence::Evidence;
use serde_json::Value;
use serde_json::json;

/// What a unit's decision read.
pub struct EvidenceJson;

impl EvidenceJson {
    pub fn value(&self, evidence: &Evidence) -> Value {
        json!({
            "line_in_use": evidence.line_in_use.map(|line| json!({
                "line": line.line.to_string(),
                "first_release": line.first_release.map(|date| date.to_string()),
                "last_release": line.last_release.map(|date| date.to_string()),
                "releases": line.releases,
            })),
            "rust_version": evidence.rust_version.as_ref().map(ToString::to_string),
            "advisories": evidence.advisories,
            "yanked_in_use": evidence.yanked_in_use,
            "footprint": evidence.footprint.iter().map(ToString::to_string).collect::<Vec<String>>(),
            "held_back": evidence.held_back,
            "held": evidence.held.iter().map(|held| MoveJson.value(held)).collect::<Vec<Value>>(),
        })
    }
}

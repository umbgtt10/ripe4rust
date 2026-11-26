// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::writing::step::Step;
use serde_json::Value;
use serde_json::json;

/// One step: `{ "run": ... }`, or `{ "edit": "file:line", "key": ..., "value": ... }`.
pub struct StepJson;

impl StepJson {
    pub fn value(&self, step: &Step) -> Value {
        match step {
            Step::Edit { at, key, value } => json!({
                "edit": at.to_string(),
                "key": key,
                "value": value,
            }),
            Step::Run { command } => json!({ "run": command }),
        }
    }
}

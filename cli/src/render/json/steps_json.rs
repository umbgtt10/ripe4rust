// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::step_json::StepJson;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::planning::writing::steps::Steps;
use serde_json::Value;
use serde_json::json;

/// A unit's steps: apply, check, gates, revert and split.
pub struct StepsJson;

impl StepsJson {
    pub fn value(&self, steps: &Steps) -> Value {
        let apply = |steps: &[Step]| {
            steps
                .iter()
                .map(|step| StepJson.value(step))
                .collect::<Vec<Value>>()
        };
        json!({
            "apply": apply(&steps.apply),
            "check": steps.check,
            "gates": steps.gates,
            "revert": { "restore": steps.revert.restore, "run": steps.revert.run },
            "split": steps
                .split
                .iter()
                .map(|split| json!({ "id": split.id, "apply": apply(&split.apply) }))
                .collect::<Vec<Value>>(),
        })
    }
}

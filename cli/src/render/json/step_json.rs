// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::step_document::StepDocument;
use ripe4rust_core::planning::writing::step::Step;

/// One step: `{ "run": ... }`, or `{ "edit": "file:line", "key": ..., "value": ... }`.
pub struct StepJson;

impl StepJson {
    pub fn document<'a>(&self, step: &'a Step) -> StepDocument<'a> {
        match step {
            Step::Edit { at, key, value } => StepDocument::Edit {
                edit: at.to_string(),
                key,
                value,
            },
            Step::Run { command } => StepDocument::Run { run: command },
        }
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::revert_document::RevertDocument;
use crate::render::json::documents::split_document::SplitDocument;
use crate::render::json::documents::step_document::StepDocument;
use crate::render::json::documents::steps_document::StepsDocument;
use crate::render::json::step_json::StepJson;
use ripe4rust_core::planning::writing::step::Step;
use ripe4rust_core::planning::writing::steps::Steps;

/// A unit's steps: apply, check, gates, revert and split.
pub struct StepsJson;

impl StepsJson {
    pub fn document<'a>(&self, steps: &'a Steps) -> StepsDocument<'a> {
        StepsDocument {
            apply: Self::applied(&steps.apply),
            check: &steps.check,
            gates: &steps.gates,
            revert: RevertDocument {
                restore: &steps.revert.restore,
                run: &steps.revert.run,
            },
            split: steps
                .split
                .iter()
                .map(|split| SplitDocument {
                    id: &split.id,
                    apply: Self::applied(&split.apply),
                })
                .collect(),
        }
    }

    fn applied(steps: &[Step]) -> Vec<StepDocument<'_>> {
        steps.iter().map(|step| StepJson.document(step)).collect()
    }
}

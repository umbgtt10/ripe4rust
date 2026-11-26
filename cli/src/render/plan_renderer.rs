// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;

/// A way of printing a run's one document: the plan, or why there is none.
pub trait PlanRenderer {
    fn failure(&self, failure: &PlanFailure) -> String;
    fn plan(&self, plan: &Plan) -> String;
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

/// A way of printing a run's one document: a command's result -- the plan,
/// what `check` found --, or why there is none.
pub trait DocumentRenderer {
    fn check(&self, report: &CheckReport) -> String;
    fn failure(&self, failure: &PlanFailure) -> String;
    fn plan(&self, plan: &Plan) -> String;
}

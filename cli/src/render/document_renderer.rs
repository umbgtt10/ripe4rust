// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::guiding::guide_text::GuideText;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;

/// A way of printing a run's one document: a command's result -- the plan,
/// what `check` found, the pins `hold` prints, the guide --, or why there is
/// none.
pub trait DocumentRenderer {
    fn check(&self, report: &CheckReport) -> String;
    fn failure(&self, failure: &PlanFailure) -> String;
    fn guide(&self, text: &GuideText) -> String;
    fn hold(&self, report: &HoldReport) -> String;
    fn plan(&self, plan: &Plan) -> String;
}

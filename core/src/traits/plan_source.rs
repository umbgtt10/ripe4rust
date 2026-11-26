// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::recorded_plan::RecordedPlan;

/// The plan a round works from, as `plan` wrote it: one of a schema this
/// release reads, or a failure saying why it is not.
pub trait PlanSource {
    fn plan(&self) -> Result<RecordedPlan, PortFailure>;
}

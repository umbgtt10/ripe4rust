// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::traits::plan_source::PlanSource;

pub struct FakePlan {
    pub plan: Result<RecordedPlan, PortFailure>,
}

impl PlanSource for FakePlan {
    fn plan(&self) -> Result<RecordedPlan, PortFailure> {
        self.plan.clone()
    }
}

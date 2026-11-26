// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::plan::Plan;
use crate::planning::plan_failure::PlanFailure;
use crate::planning::plan_writer::PlanWriter;
use crate::planning::ports::Ports;
use crate::planning::survey::Survey;

/// `plan`: every port read, then the plan made from what they answered.
pub struct Planner<'a> {
    ports: Ports<'a>,
}

impl<'a> Planner<'a> {
    pub fn new(ports: Ports<'a>) -> Self {
        Self { ports }
    }

    pub fn plan(&self) -> Result<Plan, PlanFailure> {
        let surveyed = Survey::new(self.ports).survey()?;
        Ok(PlanWriter::new(&surveyed).plan())
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::request::Request;
use crate::command::round_adapters::RoundAdapters;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::checker::Checker;

/// `check`: the core's checker run over the round's adapters, for the unit
/// the line named.
pub struct CheckCommand<'a> {
    seams: Seams<'a>,
}

impl<'a> CheckCommand<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn check(&self, request: &Request) -> Result<CheckReport, PlanFailure> {
        let unit = request.unit.as_deref().unwrap_or_default();
        RoundAdapters::new(self.seams.clone()).with(
            request.plan.as_deref().unwrap_or_default(),
            |source, files| Checker::new(source, files).check(unit),
        )
    }
}

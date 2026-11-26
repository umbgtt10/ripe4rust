// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::request::Request;
use crate::command::round_adapters::RoundAdapters;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::holding::hold_report::HoldReport;
use ripe4rust_core::planning::round::holding::holder::Holder;

/// `hold`: the core's holder run over the round's adapters, with the unit,
/// the reason and the review date the line gave.
pub struct HoldCommand<'a> {
    seams: Seams<'a>,
}

impl<'a> HoldCommand<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn hold(&self, request: &Request) -> Result<HoldReport, PlanFailure> {
        let (unit, because) = (
            request.unit.as_deref().unwrap_or_default(),
            request.because.as_deref().unwrap_or_default(),
        );
        RoundAdapters::new(self.seams.clone()).with(
            request.plan.as_deref().unwrap_or_default(),
            |source, files| Holder::new(source, files).hold(unit, because, request.review),
        )
    }
}

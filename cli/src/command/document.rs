// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::format::Format;
use crate::render::document_renderer::DocumentRenderer;
use crate::render::json::json_renderer::JsonRenderer;
use crate::render::table::table_renderer::TableRenderer;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;

/// The one document a run prints, in the format asked for, on success and
/// on failure alike.
pub struct Document {
    format: Format,
    command: String,
}

impl Document {
    pub fn new(format: Format, command: &str) -> Self {
        Self {
            format,
            command: String::from(command),
        }
    }

    pub fn check(&self, report: &CheckReport) -> String {
        self.renderer().check(report)
    }

    pub fn failure(&self, failure: &PlanFailure) -> String {
        self.renderer().failure(failure)
    }

    pub fn plan(&self, plan: &Plan) -> String {
        self.renderer().plan(plan)
    }

    fn renderer(&self) -> Box<dyn DocumentRenderer> {
        match self.format {
            Format::Json => Box::new(JsonRenderer::new(&self.command)),
            Format::Table => Box::new(TableRenderer),
        }
    }
}

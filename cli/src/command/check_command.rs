// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::cargo_layout::CargoLayout;
use crate::adapters::workspace::manifest_files::ManifestFiles;
use crate::adapters::workspace::plan::plan_file::PlanFile;
use crate::command::request::Request;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::checking::check_report::CheckReport;
use ripe4rust_core::planning::round::checking::checker::Checker;

/// `check`: the plan file read from where `--plan` names it, from the
/// directory the command runs in, the workspace's files over the run's
/// seams, and the core's checker run over them.
pub struct CheckCommand<'a> {
    seams: Seams<'a>,
}

impl<'a> CheckCommand<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn check(&self, request: &Request) -> Result<CheckReport, PlanFailure> {
        let seams = &self.seams;
        let plan = request.plan.as_deref().unwrap_or_default();
        let layout = CargoLayout::new(seams.runner, seams.dir.clone());
        let files = ManifestFiles::new(&layout, seams.files);
        let source = PlanFile::new(seams.files, seams.dir.join(plan), plan);
        Checker::new(&source, &files).check(request.unit.as_deref().unwrap_or_default())
    }
}

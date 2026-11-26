// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::cargo_layout::CargoLayout;
use crate::adapters::workspace::manifest_files::ManifestFiles;
use crate::adapters::workspace::plan::plan_file::PlanFile;
use crate::command::seams::Seams;
use ripe4rust_core::traits::plan_source::PlanSource;
use ripe4rust_core::traits::workspace_files::WorkspaceFiles;

/// The adapters `check` and `hold` read through, set up over the run's
/// seams: the plan file `--plan` names, from the directory the command runs
/// in or by an absolute path, and the workspace's files where its layout says
/// they are.
pub struct RoundAdapters<'a> {
    seams: Seams<'a>,
}

impl<'a> RoundAdapters<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn with<T>(
        &self,
        plan: &str,
        run: impl FnOnce(&dyn PlanSource, &dyn WorkspaceFiles) -> T,
    ) -> T {
        let seams = &self.seams;
        let layout = CargoLayout::new(seams.runner, seams.dir.clone());
        let files = ManifestFiles::new(&layout, seams.files);
        let source = PlanFile::new(seams.files, seams.dir.join(plan), plan);
        run(&source, &files)
    }
}

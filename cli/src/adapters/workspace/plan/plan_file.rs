// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::plan::plan_reader::PlanReader;
use crate::system::model::file_failure::FileFailure;
use crate::system::traits::file_system::FileSystem;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::traits::plan_source::PlanSource;
use std::path::PathBuf;

/// The plan file `--plan` names, read through the file system: one not
/// there, or one the system refuses, is `plan-unreadable`, named as given.
pub struct PlanFile<'a> {
    files: &'a dyn FileSystem,
    path: PathBuf,
    name: String,
}

impl<'a> PlanFile<'a> {
    pub fn new(files: &'a dyn FileSystem, path: PathBuf, name: &str) -> Self {
        Self {
            files,
            path,
            name: String::from(name),
        }
    }
}

impl PlanSource for PlanFile<'_> {
    fn plan(&self) -> Result<RecordedPlan, PortFailure> {
        let text = self
            .files
            .read(&self.path)
            .map_err(|failure| PortFailure::PlanUnreadable {
                file: self.name.clone(),
                detail: match failure {
                    FileFailure::Missing => String::from("it is not there"),
                    FileFailure::Failed { detail } => detail,
                },
            })?;
        PlanReader.plan(&self.name, &text)
    }
}

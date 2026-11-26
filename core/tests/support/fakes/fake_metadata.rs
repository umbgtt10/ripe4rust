// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::traits::workspace_metadata::WorkspaceMetadata;

pub struct FakeMetadata {
    pub workspace: Workspace,
    pub reached: Vec<(String, Vec<String>)>,
    pub failure: Option<PortFailure>,
}

impl WorkspaceMetadata for FakeMetadata {
    fn read(&self) -> Result<Workspace, PortFailure> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.workspace.clone()),
        }
    }

    fn reached_by(&self, target: &str) -> Result<Vec<String>, PortFailure> {
        Ok(self
            .reached
            .iter()
            .find(|(known, _)| known == target)
            .map(|(_, crates)| crates.clone())
            .unwrap_or_default())
    }
}

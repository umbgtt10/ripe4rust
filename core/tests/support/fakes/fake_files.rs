// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::traits::workspace_files::WorkspaceFiles;
use ripe4rust_core::values::location::Location;

pub struct FakeFiles {
    pub fingerprint: String,
    pub lock: Vec<LockEntry>,
    pub policy: PolicyTree,
    pub toolchain: Option<ToolchainFile>,
    pub requirement_lines: Vec<(String, Vec<Location>)>,
    pub rust_version_lines: Vec<(String, Location)>,
    pub written: Vec<WrittenLine>,
    pub failure: Option<PortFailure>,
}

impl WorkspaceFiles for FakeFiles {
    fn fingerprint(&self) -> Result<String, PortFailure> {
        Ok(self.fingerprint.clone())
    }

    fn lock(&self) -> Result<Vec<LockEntry>, PortFailure> {
        Ok(self.lock.clone())
    }

    fn policy(&self) -> Result<PolicyTree, PortFailure> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.policy.clone()),
        }
    }

    fn requirement_lines(&self, name: &str) -> Result<Vec<Location>, PortFailure> {
        Ok(self
            .requirement_lines
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, lines)| lines.clone())
            .unwrap_or_default())
    }

    fn rust_version_line(&self, member: &str) -> Result<Option<Location>, PortFailure> {
        Ok(self
            .rust_version_lines
            .iter()
            .find(|(known, _)| known == member)
            .map(|(_, at)| at.clone()))
    }

    fn toolchain(&self) -> Result<Option<ToolchainFile>, PortFailure> {
        Ok(self.toolchain.clone())
    }

    fn written(&self) -> Result<Vec<WrittenLine>, PortFailure> {
        Ok(self.written.clone())
    }
}

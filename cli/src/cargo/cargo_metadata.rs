// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cargo::cargo_command::CargoCommand;
use crate::cargo::metadata_reader::MetadataReader;
use crate::traits::command_runner::CommandRunner;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::traits::workspace_metadata::WorkspaceMetadata;
use std::path::PathBuf;

/// `cargo metadata --format-version 1 --locked`: the members and the lock,
/// and with `--filter-platform` which crates a target's build resolves.
/// `--locked` keeps the lock file as it is, and `--offline` the network out
/// of an offline plan.
pub struct CargoMetadata<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
    offline: bool,
}

impl<'a> CargoMetadata<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf, offline: bool) -> Self {
        Self {
            runner,
            dir,
            offline,
        }
    }

    fn metadata(&self, filter: &[&str]) -> Result<String, PortFailure> {
        let mut args = ["metadata", "--format-version", "1", "--locked"]
            .map(String::from)
            .to_vec();
        if self.offline {
            args.push(String::from("--offline"));
        }
        args.extend(filter.iter().map(|word| String::from(*word)));
        let output = CargoCommand::new(self.runner, self.dir.clone())
            .run(args, |detail| PortFailure::CargoMetadataFailed { detail })?;
        Ok(output.stdout)
    }
}

impl WorkspaceMetadata for CargoMetadata<'_> {
    fn read(&self) -> Result<Workspace, PortFailure> {
        MetadataReader.workspace(&self.metadata(&[])?)
    }

    fn reached_by(&self, target: &str) -> Result<Vec<String>, PortFailure> {
        MetadataReader.registry_crates(&self.metadata(&["--filter-platform", target])?)
    }
}

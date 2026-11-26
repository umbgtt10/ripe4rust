// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::complaint::Complaint;
use crate::adapters::unreadable::Unreadable;
use crate::system::model::command_output::CommandOutput;
use crate::system::model::invocation::Invocation;
use crate::system::traits::command_runner::CommandRunner;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::traits::toolchain::Toolchain;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

const RUSTC: &str = "rustc";

/// The toolchain as `rustc` in the workspace's directory knows it --
/// `rust-toolchain.toml` choosing it there --: its version and the targets
/// it knows. Its releases, the channel manifests, are slice 5's to read:
/// until then it knows none, and the plan says so.
pub struct RustcToolchain<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
}

impl<'a> RustcToolchain<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf) -> Self {
        Self { runner, dir }
    }

    fn run(&self, args: &[&str]) -> Result<CommandOutput, PortFailure> {
        let output = self
            .runner
            .run(&Invocation {
                program: String::from(RUSTC),
                args: args.iter().map(|arg| String::from(*arg)).collect(),
                dir: self.dir.clone(),
            })
            .map_err(|failure| PortFailure::ToolchainUnreachable {
                detail: Complaint.unlaunched(&failure),
            })?;
        if output.status == Some(0) {
            Ok(output)
        } else {
            Err(PortFailure::ToolchainUnreachable {
                detail: Complaint.of(&output),
            })
        }
    }
}

impl Toolchain for RustcToolchain<'_> {
    fn releases(&self) -> Result<Option<Vec<Release>>, PortFailure> {
        Ok(None)
    }

    fn rustc(&self) -> Result<Version, PortFailure> {
        let output = self.run(&["--version"])?;
        let line = output.stdout.trim();
        let unreadable = Unreadable::new("reading what `rustc --version` printed");
        let version = line
            .split_whitespace()
            .nth(1)
            .ok_or_else(|| unreadable.failure(&format!("`{line}` names no version")))?;
        Version::parse(version).map_err(|failure| unreadable.unparsed(&failure))
    }

    fn targets(&self) -> Result<Vec<String>, PortFailure> {
        Ok(self
            .run(&["--print", "target-list"])?
            .stdout
            .lines()
            .map(str::trim)
            .filter(|target| !target.is_empty())
            .map(String::from)
            .collect())
    }
}

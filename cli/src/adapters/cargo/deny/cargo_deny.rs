// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::deny::audit_report::AuditReport;
use crate::adapters::cargo::deny::deny_log::DenyLog;
use crate::adapters::complaint::Complaint;
use crate::system::model::invocation::Invocation;
use crate::system::traits::command_runner::CommandRunner;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::advisories::Advisories;
use std::path::PathBuf;

const CARGO: &str = "cargo";
const NOT_INSTALLED: &str = "no such command: `deny`";

/// `cargo deny --format json --locked --workspace check
/// --audit-compatible-output advisories`: the advisories in force from its
/// log, which leaves out those `deny.toml` ignores, and where each is clear
/// from its audit report. It exits 1 when it found advisories, so 0 and 1
/// are a check made; an error in its log, or any other end, is a check it
/// could not make. `--locked` keeps the lock file, `--offline` the network
/// out of an offline plan.
pub struct CargoDeny<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
    offline: bool,
}

impl<'a> CargoDeny<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf, offline: bool) -> Self {
        Self {
            runner,
            dir,
            offline,
        }
    }

    fn args(&self) -> Vec<String> {
        let mut args = ["deny", "--format", "json", "--locked"]
            .map(String::from)
            .to_vec();
        if self.offline {
            args.push(String::from("--offline"));
        }
        args.extend(
            [
                "--workspace",
                "check",
                "--audit-compatible-output",
                "advisories",
            ]
            .map(String::from),
        );
        args
    }
}

impl Advisories for CargoDeny<'_> {
    fn advisories(&self) -> Result<Vec<Advisory>, PortFailure> {
        let output = self
            .runner
            .run(&Invocation {
                program: String::from(CARGO),
                args: self.args(),
                dir: self.dir.clone(),
            })
            .map_err(|failure| PortFailure::CargoDenyFailed {
                detail: Complaint.unlaunched(&failure),
            })?;
        if output.stderr.contains(NOT_INSTALLED) {
            return Err(PortFailure::CargoDenyMissing);
        }
        let findings = DenyLog.read(&output.stderr);
        match (output.status, findings.errors.first()) {
            (_, Some(error)) => Err(PortFailure::CargoDenyFailed {
                detail: error.clone(),
            }),
            (Some(0 | 1), None) => AuditReport.advisories(&output.stdout, &findings.hits),
            (_, None) => Err(PortFailure::CargoDenyFailed {
                detail: Complaint.of(&output),
            }),
        }
    }
}

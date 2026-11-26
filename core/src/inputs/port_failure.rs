// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use alloc::format;
use alloc::string::String;

const ISSUES: &str = "https://github.com/umbgtt10/ripe4rust/issues";

/// Why a port could not answer: every one stops the plan, as the diagnostic
/// of its own code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortFailure {
    CargoDenyFailed { detail: String },
    CargoDenyMissing,
    CargoMetadataFailed { detail: String },
    CargoUpdateFailed { detail: String },
    CargoUpdateUnrecognised { line: String },
    Internal { doing: String, detail: String },
    RegistryUnreachable { detail: String },
    ToolchainUnreachable { detail: String },
    WorkspaceUnreadable { file: String, detail: String },
}

impl PortFailure {
    pub fn diagnostic(&self) -> Diagnostic {
        let (code, what, action) = match self {
            Self::CargoDenyFailed { detail } => (
                DiagnosticCode::CargoDenyFailed,
                format!("cargo-deny could not check the advisories: {detail}"),
                String::from(
                    "run `cargo deny check advisories`, fix what stops it, then plan again",
                ),
            ),
            Self::CargoDenyMissing => (
                DiagnosticCode::CargoDenyMissing,
                String::from(
                    "cargo-deny is not installed, and a plan blind to advisories would call ripe what is urgent",
                ),
                String::from(
                    "install it with `cargo install --locked cargo-deny`, then plan again",
                ),
            ),
            Self::CargoMetadataFailed { detail } => (
                DiagnosticCode::CargoMetadataFailed,
                format!("`cargo metadata` could not run: {detail}"),
                String::from("fix what stops `cargo metadata` in the workspace, then plan again"),
            ),
            Self::CargoUpdateFailed { detail } => (
                DiagnosticCode::CargoUpdateFailed,
                format!("`cargo update --dry-run` could not run: {detail}"),
                String::from(
                    "fix what stops `cargo update --dry-run` in the workspace, then plan again",
                ),
            ),
            Self::CargoUpdateUnrecognised { line } => (
                DiagnosticCode::CargoUpdateUnrecognised,
                format!(
                    "`cargo update --dry-run` printed a line ripe4rust does not know: `{line}`"
                ),
                format!(
                    "report it at {ISSUES} with the line; no plan is made that could leave a move out"
                ),
            ),
            Self::Internal { doing, detail } => (
                DiagnosticCode::Internal,
                format!("ripe4rust failed while {doing}: {detail}"),
                format!("report it at {ISSUES} with this diagnostic"),
            ),
            Self::RegistryUnreachable { detail } => (
                DiagnosticCode::RegistryUnreachable,
                format!(
                    "the registry could not be reached, and nothing is cached for the day: {detail}"
                ),
                String::from(
                    "check the network and plan again, or plan with `--offline` to read an earlier day's cache",
                ),
            ),
            Self::ToolchainUnreachable { detail } => (
                DiagnosticCode::ToolchainUnreachable,
                format!(
                    "the toolchain's releases could not be read, and nothing is cached for the day: {detail}"
                ),
                String::from("check the network and `rustc`, then plan again"),
            ),
            Self::WorkspaceUnreadable { file, detail } => (
                DiagnosticCode::WorkspaceUnreadable,
                format!("`{file}` could not be read: {detail}"),
                format!("make `{file}` readable, then plan again"),
            ),
        };
        code.diagnose(None, what, action)
    }
}

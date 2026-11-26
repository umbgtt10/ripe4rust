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
    CurlMissing,
    Internal { doing: String, detail: String },
    LockOutdated { detail: String },
    PlanSchemaUnknown { file: String, schema: String },
    PlanUnreadable { file: String, detail: String },
    RegistryCacheUnusable { detail: String },
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
                String::from(DiagnosticCode::CargoDenyMissing.entry().remedy),
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
            Self::CurlMissing => (
                DiagnosticCode::CurlMissing,
                String::from("curl is not installed, and ripe4rust reads the registry through it"),
                String::from(
                    "install curl -- Windows 10 and later, macOS and most Linux distributions ship it -- or put it on the PATH, then plan again; or plan with `--offline` to read the cache alone",
                ),
            ),
            Self::Internal { doing, detail } => (
                DiagnosticCode::Internal,
                format!("ripe4rust failed while {doing}: {detail}"),
                format!("report it at {ISSUES} with this diagnostic"),
            ),
            Self::LockOutdated { detail } => (
                DiagnosticCode::LockOutdated,
                format!(
                    "`Cargo.lock` is not there, or no longer matches the workspace's manifests, and ripe4rust writes no lock: {detail}"
                ),
                String::from(DiagnosticCode::LockOutdated.entry().remedy),
            ),
            Self::PlanSchemaUnknown { file, schema } => (
                DiagnosticCode::PlanSchemaUnknown,
                format!(
                    "`{file}` is a plan of schema {schema}, and this release of ripe4rust reads schema 1"
                ),
                format!(
                    "plan again with this release -- `cargo ripe4rust plan --format json > {file}` -- and go on with the new plan"
                ),
            ),
            Self::PlanUnreadable { file, detail } => (
                DiagnosticCode::PlanUnreadable,
                format!("`{file}` could not be read as a plan: {detail}"),
                format!(
                    "write the plan with `cargo ripe4rust plan --format json > {file}`, and run the command again"
                ),
            ),
            Self::RegistryCacheUnusable { detail } => (
                DiagnosticCode::RegistryCacheUnusable,
                format!("the registry's answers could not be cached: {detail}"),
                String::from(DiagnosticCode::RegistryCacheUnusable.entry().remedy),
            ),
            Self::RegistryUnreachable { detail } => (
                DiagnosticCode::RegistryUnreachable,
                format!(
                    "the registry could not be reached, and nothing is cached for the day: {detail}"
                ),
                String::from(DiagnosticCode::RegistryUnreachable.entry().remedy),
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

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::severity::Severity;
use crate::values::location::Location;
use alloc::string::String;

/// One kind of problem, with its stable code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCode {
    CargoDenyFailed,
    CargoDenyMissing,
    CargoMetadataFailed,
    CargoUpdateFailed,
    CargoUpdateUnrecognised,
    CommandUnavailable,
    CurlMissing,
    GatesUndeclared,
    Internal,
    PinForAbsentCrate,
    PinWithoutReason,
    PinWithoutRelease,
    PlanSchemaUnknown,
    PlanStale,
    PlanUnreadable,
    PolicyInvalidValue,
    PolicyUnknownKey,
    RegistryCacheOld,
    RegistryCacheUnusable,
    RegistryUnreachable,
    SourceUnparsed,
    SourceUnscanned,
    TargetUnknown,
    ToolchainReleasesUnknown,
    ToolchainUnpinned,
    ToolchainUnreachable,
    UnitUnknown,
    UnitUrgent,
    UsageInvalid,
    WorkspaceUnreadable,
}

impl DiagnosticCode {
    pub fn diagnose(self, at: Option<Location>, what: String, action: String) -> Diagnostic {
        Diagnostic {
            severity: self.severity(),
            code: self,
            at,
            what,
            action,
            needs_person: self.needs_person(),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::CargoDenyFailed => "cargo-deny-failed",
            Self::CargoDenyMissing => "cargo-deny-missing",
            Self::CargoMetadataFailed => "cargo-metadata-failed",
            Self::CargoUpdateFailed => "cargo-update-failed",
            Self::CargoUpdateUnrecognised => "cargo-update-unrecognised",
            Self::CommandUnavailable => "command-unavailable",
            Self::CurlMissing => "curl-missing",
            Self::GatesUndeclared => "gates-undeclared",
            Self::Internal => "internal",
            Self::PinForAbsentCrate => "pin-for-absent-crate",
            Self::PinWithoutReason => "pin-without-reason",
            Self::PinWithoutRelease => "pin-without-release",
            Self::PlanSchemaUnknown => "plan-schema-unknown",
            Self::PlanStale => "plan-stale",
            Self::PlanUnreadable => "plan-unreadable",
            Self::PolicyInvalidValue => "policy-invalid-value",
            Self::PolicyUnknownKey => "policy-unknown-key",
            Self::RegistryCacheOld => "registry-cache-old",
            Self::RegistryCacheUnusable => "registry-cache-unusable",
            Self::RegistryUnreachable => "registry-unreachable",
            Self::SourceUnparsed => "source-unparsed",
            Self::SourceUnscanned => "source-unscanned",
            Self::TargetUnknown => "target-unknown",
            Self::ToolchainReleasesUnknown => "toolchain-releases-unknown",
            Self::ToolchainUnpinned => "toolchain-unpinned",
            Self::ToolchainUnreachable => "toolchain-unreachable",
            Self::UnitUnknown => "unit-unknown",
            Self::UnitUrgent => "unit-urgent",
            Self::UsageInvalid => "usage-invalid",
            Self::WorkspaceUnreadable => "workspace-unreadable",
        }
    }

    pub fn needs_person(&self) -> bool {
        matches!(
            self,
            Self::GatesUndeclared
                | Self::PinWithoutReason
                | Self::PinWithoutRelease
                | Self::PolicyInvalidValue
                | Self::PolicyUnknownKey
                | Self::TargetUnknown
                | Self::UnitUrgent
        )
    }

    pub fn severity(&self) -> Severity {
        match self {
            Self::GatesUndeclared
            | Self::PinForAbsentCrate
            | Self::RegistryCacheOld
            | Self::SourceUnparsed => Severity::Warning,
            Self::SourceUnscanned | Self::ToolchainReleasesUnknown | Self::ToolchainUnpinned => {
                Severity::Notice
            }
            _ => Severity::Error,
        }
    }
}

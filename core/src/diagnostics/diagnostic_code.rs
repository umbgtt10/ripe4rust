// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::code_entry::CodeEntry;
use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::severity::Severity;
use crate::values::location::Location;
use alloc::string::String;

const UNLISTED: CodeEntry = CodeEntry {
    means: "",
    remedy: "",
};
const ENTRIES: [(DiagnosticCode, CodeEntry); 30] = [
    (
        DiagnosticCode::CargoDenyFailed,
        CodeEntry {
            means: "cargo-deny could not check the advisories, and a plan blind to them would call ripe what is urgent",
            remedy: "run `cargo deny check advisories`, fix what stops it -- its error is quoted --, then plan again",
        },
    ),
    (
        DiagnosticCode::CargoDenyMissing,
        CodeEntry {
            means: "cargo-deny is not installed, and a plan blind to advisories would call ripe what is urgent",
            remedy: "install it with `cargo install --locked cargo-deny`, then plan again",
        },
    ),
    (
        DiagnosticCode::CargoMetadataFailed,
        CodeEntry {
            means: "`cargo metadata` could not run in the workspace",
            remedy: "fix what stops `cargo metadata` -- its error is quoted --, then plan again",
        },
    ),
    (
        DiagnosticCode::CargoUpdateFailed,
        CodeEntry {
            means: "`cargo update --dry-run` could not run in the workspace",
            remedy: "fix what stops `cargo update --dry-run` -- its error is quoted --, then plan again",
        },
    ),
    (
        DiagnosticCode::CargoUpdateUnrecognised,
        CodeEntry {
            means: "`cargo update --dry-run` printed a line ripe4rust does not know, and no plan is made that could leave a move out",
            remedy: "report it at https://github.com/umbgtt10/ripe4rust/issues with the line",
        },
    ),
    (
        DiagnosticCode::CurlMissing,
        CodeEntry {
            means: "curl is not on the PATH, and ripe4rust reads the registry through it",
            remedy: "install curl -- Windows 10 and later, macOS and most Linux distributions ship it -- or put it on the PATH; or plan with `--offline` to read the cache alone",
        },
    ),
    (
        DiagnosticCode::GatesUndeclared,
        CodeEntry {
            means: "no `gates` are declared, so nothing can validate a unit: every unit that would be ripe is asked",
            remedy: "ask the owner for the commands that validate a change, and write them as `gates = [...]` in `[workspace.metadata.ripe4rust]`",
        },
    ),
    (
        DiagnosticCode::Internal,
        CodeEntry {
            means: "ripe4rust failed in a way of its own, saying what it was doing",
            remedy: "report it at https://github.com/umbgtt10/ripe4rust/issues with the diagnostic",
        },
    ),
    (
        DiagnosticCode::LockOutdated,
        CodeEntry {
            means: "`Cargo.lock` is not there, or no longer matches the workspace's manifests, and ripe4rust writes no lock: `cargo metadata --locked` refuses to bring it in step",
            remedy: "bring the lock in step with the manifests -- `cargo update --workspace` writes it and moves no locked version they still allow --, commit it, then plan again",
        },
    ),
    (
        DiagnosticCode::PinForAbsentCrate,
        CodeEntry {
            means: "a pin names a crate no longer in the tree, so it holds nothing",
            remedy: "remove the pin from `[workspace.metadata.ripe4rust.pins]`",
        },
    ),
    (
        DiagnosticCode::PinWithoutReason,
        CodeEntry {
            means: "a pin gives no reason",
            remedy: "ask the owner why the crate is held, and write it as the pin's `reason`",
        },
    ),
    (
        DiagnosticCode::PinWithoutRelease,
        CodeEntry {
            means: "a pin has neither a review date nor a release condition that can be checked",
            remedy: "ask the owner when to look at it again, and write it as the pin's `review = \"YYYY-MM-DD\"` or `until = { crate = \"...\", version = \">=...\" }`",
        },
    ),
    (
        DiagnosticCode::PlanSchemaUnknown,
        CodeEntry {
            means: "the plan file is of a `schema` this release of ripe4rust does not read",
            remedy: "plan again with this release -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan",
        },
    ),
    (
        DiagnosticCode::PlanStale,
        CodeEntry {
            means: "the workspace changed outside the plan: a change no unit of it explains",
            remedy: "plan again -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan",
        },
    ),
    (
        DiagnosticCode::PlanUnreadable,
        CodeEntry {
            means: "the plan file `--plan` names is not there, or is no plan `plan` wrote",
            remedy: "write the plan with `cargo ripe4rust plan --format json > plan.json`, and run the command again",
        },
    ),
    (
        DiagnosticCode::PolicyInvalidValue,
        CodeEntry {
            means: "a key of `[workspace.metadata.ripe4rust]` holds a value of the wrong kind",
            remedy: "ask the owner for the value meant, and write it as the kind the diagnostic names",
        },
    ),
    (
        DiagnosticCode::PolicyUnknownKey,
        CodeEntry {
            means: "`[workspace.metadata.ripe4rust]` holds a key ripe4rust does not know, which would otherwise be ignored without a word",
            remedy: "ask the owner what was meant: correct its spelling or remove it; a key newer than this release needs a newer ripe4rust",
        },
    ),
    (
        DiagnosticCode::RegistryCacheOld,
        CodeEntry {
            means: "`--offline` read an earlier day's cache: what was published since is not in the plan",
            remedy: "plan again online to read today's registry",
        },
    ),
    (
        DiagnosticCode::RegistryCacheUnusable,
        CodeEntry {
            means: "the registry's day cache cannot be read or written, or there is no directory for it",
            remedy: "make the cache directory readable and writable, or set `XDG_CACHE_HOME` (`LOCALAPPDATA` on Windows) to one that is, then plan again",
        },
    ),
    (
        DiagnosticCode::RegistryUnreachable,
        CodeEntry {
            means: "the registry could not be reached, and nothing is cached for the day",
            remedy: "check the network and plan again, or plan with `--offline` to read an earlier day's cache",
        },
    ),
    (
        DiagnosticCode::SourceUnparsed,
        CodeEntry {
            means: "a source file the parse could not read: the footprint of the crates it names may be low",
            remedy: "fix the file so it parses, then plan again; or take the footprint as a floor",
        },
    ),
    (
        DiagnosticCode::SourceUnscanned,
        CodeEntry {
            means: "the sources are not parsed yet: no breaking unit's footprint or exposure is known",
            remedy: "nothing is needed; size each breaking unit's migration by hand",
        },
    ),
    (
        DiagnosticCode::TargetUnknown,
        CodeEntry {
            means: "a target in `validate-beyond-gates` the toolchain does not know",
            remedy: "ask the owner which target was meant, and correct or remove it; `rustc --print target-list` lists the targets the toolchain knows",
        },
    ),
    (
        DiagnosticCode::ToolchainReleasesUnknown,
        CodeEntry {
            means: "the toolchain's releases are not read yet: no toolchain unit is offered",
            remedy: "nothing is needed; compare `rust-toolchain.toml`'s channel with the stable releases by hand",
        },
    ),
    (
        DiagnosticCode::ToolchainUnpinned,
        CodeEntry {
            means: "`rust-toolchain.toml` pins no stable release, or is not there: there is no toolchain unit",
            remedy: "nothing is needed; to have the toolchain judged as a unit, pin a stable release in its `channel`",
        },
    ),
    (
        DiagnosticCode::ToolchainUnreachable,
        CodeEntry {
            means: "the toolchain's version, targets or releases could not be read",
            remedy: "check `rustc` and the network, then plan again",
        },
    ),
    (
        DiagnosticCode::UnitUnknown,
        CodeEntry {
            means: "`check` or `hold` names an id that is no unit of the plan nor a move of a unit's split, or an advisory row, which applies nothing",
            remedy: "name a unit by its `id` in the plan, or one of its moves by an `id` of its `steps.split`; an advisory row is answered, never applied",
        },
    ),
    (
        DiagnosticCode::UnitUrgent,
        CodeEntry {
            means: "`hold` names an urgent unit, which is never held: holding it would keep what it clears",
            remedy: "do not hold it: it becomes an urgent ask -- carry out its `revert` if it is applied, and put its question to the person, with the failure attached, ahead of every other",
        },
    ),
    (
        DiagnosticCode::UsageInvalid,
        CodeEntry {
            means: "a command or an option ripe4rust does not know, an option given no value or one of the wrong kind, or one a command cannot go without missing",
            remedy: "run the command as its usage says; `cargo ripe4rust guide` names every command and what each takes",
        },
    ),
    (
        DiagnosticCode::WorkspaceUnreadable,
        CodeEntry {
            means: "a file of the workspace ripe4rust reads could not be read or parsed",
            remedy: "make the file readable, or fix it where it breaks, then run the command again",
        },
    ),
];

/// One kind of problem, with its stable code: the catalogue every
/// diagnostic and `guide` come from -- its label, severity, whether only a
/// person can fix it, and its entry: what it means and its remedy. `ALL` holds
/// every code,
/// in the order of its label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticCode {
    CargoDenyFailed,
    CargoDenyMissing,
    CargoMetadataFailed,
    CargoUpdateFailed,
    CargoUpdateUnrecognised,
    CurlMissing,
    GatesUndeclared,
    Internal,
    LockOutdated,
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
    pub const ALL: [Self; 30] = [
        Self::CargoDenyFailed,
        Self::CargoDenyMissing,
        Self::CargoMetadataFailed,
        Self::CargoUpdateFailed,
        Self::CargoUpdateUnrecognised,
        Self::CurlMissing,
        Self::GatesUndeclared,
        Self::Internal,
        Self::LockOutdated,
        Self::PinForAbsentCrate,
        Self::PinWithoutReason,
        Self::PinWithoutRelease,
        Self::PlanSchemaUnknown,
        Self::PlanStale,
        Self::PlanUnreadable,
        Self::PolicyInvalidValue,
        Self::PolicyUnknownKey,
        Self::RegistryCacheOld,
        Self::RegistryCacheUnusable,
        Self::RegistryUnreachable,
        Self::SourceUnparsed,
        Self::SourceUnscanned,
        Self::TargetUnknown,
        Self::ToolchainReleasesUnknown,
        Self::ToolchainUnpinned,
        Self::ToolchainUnreachable,
        Self::UnitUnknown,
        Self::UnitUrgent,
        Self::UsageInvalid,
        Self::WorkspaceUnreadable,
    ];

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

    pub fn entry(&self) -> CodeEntry {
        ENTRIES
            .iter()
            .find(|(code, _)| code == self)
            .map_or(UNLISTED, |(_, entry)| *entry)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::CargoDenyFailed => "cargo-deny-failed",
            Self::CargoDenyMissing => "cargo-deny-missing",
            Self::CargoMetadataFailed => "cargo-metadata-failed",
            Self::CargoUpdateFailed => "cargo-update-failed",
            Self::CargoUpdateUnrecognised => "cargo-update-unrecognised",
            Self::CurlMissing => "curl-missing",
            Self::GatesUndeclared => "gates-undeclared",
            Self::Internal => "internal",
            Self::LockOutdated => "lock-outdated",
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

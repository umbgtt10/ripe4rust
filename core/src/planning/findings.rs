// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::toolchain_file::ToolchainFile;
use crate::inputs::unparsed_file::UnparsedFile;
use crate::planning::pins_and_policy::pin::Pin;
use crate::values::date::Date;
use crate::values::location::Location;
use crate::values::version::Version;
use alloc::format;
use alloc::string::String;

/// The problems the survey itself meets, each a diagnostic saying what to do.
pub struct Findings;

impl Findings {
    pub fn pin_for_absent_crate(&self, pin: &Pin) -> Diagnostic {
        DiagnosticCode::PinForAbsentCrate.diagnose(
            Some(pin.at.clone()),
            format!(
                "the pin on `{}` names a crate no longer in the tree",
                pin.name
            ),
            String::from("remove the pin from `[workspace.metadata.ripe4rust.pins]`"),
        )
    }

    pub fn registry_cache_old(&self, read_on: Date, today: Date) -> Diagnostic {
        DiagnosticCode::RegistryCacheOld.diagnose(
            None,
            format!(
                "the registry's answers are those cached on {read_on}, not on {today}: what was published since is not in the plan"
            ),
            String::from("plan again online to read today's registry"),
        )
    }

    pub fn source_unparsed(&self, file: &UnparsedFile) -> Diagnostic {
        DiagnosticCode::SourceUnparsed.diagnose(
            None,
            format!(
                "`{}` could not be parsed ({}): the footprint of the crates it names may be low",
                file.file, file.detail
            ),
            String::from(
                "fix the file so it parses, then plan again; or take the footprint as a floor",
            ),
        )
    }

    pub fn source_unscanned(&self) -> Diagnostic {
        DiagnosticCode::SourceUnscanned.diagnose(
            None,
            String::from(
                "the workspace's sources are not parsed yet, so no breaking unit's footprint or exposure is known: none is ripe by its footprint (rule 11), and one a published member uses is asked (rule 8)",
            ),
            String::from(
                "nothing is needed; until a release of ripe4rust parses the sources, size each breaking unit's migration by hand",
            ),
        )
    }

    pub fn target_unknown(&self, target: &str, at: Option<Location>) -> Diagnostic {
        DiagnosticCode::TargetUnknown.diagnose(
            at,
            format!("`{target}` in `validate-beyond-gates` is a target the toolchain does not know"),
            String::from(
                "correct its name or remove it; `rustc --print target-list` lists the targets the toolchain knows",
            ),
        )
    }

    pub fn toolchain_releases_unknown(&self, at: Location) -> Diagnostic {
        DiagnosticCode::ToolchainReleasesUnknown.diagnose(
            Some(at),
            String::from(
                "the toolchain's releases are not read yet: no toolchain unit is offered, and the versions a newer toolchain would release are not named",
            ),
            String::from(
                "nothing is needed; until a release of ripe4rust reads them, compare `rust-toolchain.toml`'s channel with the stable releases by hand",
            ),
        )
    }

    pub fn toolchain_unpinned(&self, file: Option<&ToolchainFile>, rustc: &Version) -> Diagnostic {
        let what = match file {
            Some(file) => format!(
                "`rust-toolchain.toml`'s channel, `{}`, pins no stable release: there is no toolchain unit, and the versions are chosen for rustc {rustc}",
                file.channel
            ),
            None => format!(
                "the workspace has no `rust-toolchain.toml`: there is no toolchain unit, and the versions are chosen for rustc {rustc}"
            ),
        };
        DiagnosticCode::ToolchainUnpinned.diagnose(
            file.map(|file| file.at.clone()),
            what,
            String::from(
                "nothing is needed; to have the toolchain judged as a unit, pin a stable release in `rust-toolchain.toml`'s `channel`",
            ),
        )
    }
}

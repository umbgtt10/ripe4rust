// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::inputs::advisory::Advisory;
use crate::inputs::lock_move::LockMove;
use crate::inputs::release::Release;
use crate::inputs::scan::Scan;
use crate::inputs::workspace::Workspace;
use crate::inputs::written_line::WrittenLine;
use crate::planning::pins_and_policy::policy::Policy;
use crate::planning::units::crate_repository::CrateRepository;
use crate::planning::units::dependency_moves::DependencyMoves;
use crate::planning::units::direct_dependency::DirectDependency;
use crate::values::date::Date;
use crate::values::location::Location;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// Everything the ports answered, read once, before anything is decided: a
/// plan is made from this alone. `pinned` is the channel's release, where
/// it pins one; `toolchain` the toolchain in use; `releases` the index's
/// answer for every crate read; `reached` the crates each target needing
/// more than the gates reaches; the lines are where each direct
/// dependency's requirement and each member's `rust-version` are written,
/// and `written` every line a unit can edit with what it says; `scan` is
/// `None` where the scan failed as a whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Surveyed {
    pub today: Date,
    pub registry_read: Date,
    pub fingerprint: String,
    pub policy: Policy,
    pub workspace: Workspace,
    pub direct: Vec<DirectDependency>,
    pub pinned: Option<Version>,
    pub channel_at: Option<Location>,
    pub toolchain: Version,
    pub toolchain_releases: Vec<Release>,
    pub releases: Vec<(String, Vec<Release>)>,
    pub advisories: Vec<Advisory>,
    pub dependency_moves: Vec<DependencyMoves>,
    pub workspace_moves: Vec<LockMove>,
    pub repositories: Vec<CrateRepository>,
    pub reached: Vec<(String, Vec<String>)>,
    pub scan: Option<Scan>,
    pub requirement_lines: Vec<(String, Vec<Location>)>,
    pub rust_version_lines: Vec<(String, Location)>,
    pub written: Vec<WrittenLine>,
    pub diagnostics: Vec<Diagnostic>,
}

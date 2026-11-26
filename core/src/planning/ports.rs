// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::traits::advisories::Advisories;
use crate::traits::clock::Clock;
use crate::traits::index::Index;
use crate::traits::lock_moves::LockMoves;
use crate::traits::repositories::Repositories;
use crate::traits::source_scan::SourceScan;
use crate::traits::toolchain::Toolchain;
use crate::traits::workspace_files::WorkspaceFiles;
use crate::traits::workspace_metadata::WorkspaceMetadata;

/// Every input a plan reads, each through its port.
#[derive(Clone, Copy)]
pub struct Ports<'a> {
    pub clock: &'a dyn Clock,
    pub index: &'a dyn Index,
    pub repositories: &'a dyn Repositories,
    pub lock_moves: &'a dyn LockMoves,
    pub metadata: &'a dyn WorkspaceMetadata,
    pub files: &'a dyn WorkspaceFiles,
    pub advisories: &'a dyn Advisories,
    pub toolchain: &'a dyn Toolchain,
    pub scan: &'a dyn SourceScan,
}

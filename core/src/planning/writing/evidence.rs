// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::choice::line_stats::LineStats;
use crate::values::location::Location;
use crate::values::partial_version::PartialVersion;
use alloc::string::String;
use alloc::vec::Vec;

/// What a unit's decision read, for a caller asking a person to show why:
/// the line in use, the `rust-version` the target needs, what it clears, a
/// breaking unit's footprint -- the migration's work list --, the versions
/// the toolchain unit would release, and the moves a pin holds out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Evidence {
    pub line_in_use: Option<LineStats>,
    pub rust_version: Option<PartialVersion>,
    pub advisories: Vec<String>,
    pub yanked_in_use: bool,
    pub footprint: Vec<Location>,
    pub held_back: Vec<String>,
    pub held: Vec<LockMove>,
}

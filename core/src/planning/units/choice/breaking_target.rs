// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::line::Line;
use crate::values::version::Version;
use alloc::vec::Vec;

/// Where a breaking unit moves: the newest release of the newest settled
/// line, or of the newest line where every newer line is young.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakingTarget {
    pub to: Version,
    pub line: Line,
    pub young: bool,
    pub younger_lines: Vec<Line>,
    pub lines_behind: u32,
}

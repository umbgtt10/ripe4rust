// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::choice::line_stats::LineStats;

/// A line as of the plan's date: its releases, young or settled, active or
/// stale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineStanding {
    pub stats: LineStats,
    pub young: bool,
    pub active: bool,
}

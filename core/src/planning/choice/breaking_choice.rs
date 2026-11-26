// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::choice::breaking_target::BreakingTarget;
use crate::planning::choice::maturity::Maturity;
use crate::planning::choice::release_history::ReleaseHistory;
use crate::values::version::Version;

/// The version a breaking unit moves to: the newest release of the newest
/// line that has settled, the younger lines beyond it named; where every
/// newer line is young, the newest, for rule 9 to hold.
pub struct BreakingChoice<'a> {
    history: &'a ReleaseHistory,
    maturity: &'a Maturity,
    toolchain: &'a Version,
}

impl<'a> BreakingChoice<'a> {
    pub fn new(
        history: &'a ReleaseHistory,
        maturity: &'a Maturity,
        toolchain: &'a Version,
    ) -> Self {
        Self {
            history,
            maturity,
            toolchain,
        }
    }

    pub fn choose(&self, in_use: &Version) -> Option<BreakingTarget> {
        let lines = self
            .history
            .newer_lines(self.history.line_of(in_use), self.toolchain);
        let newest = *lines.last()?;
        let line = lines
            .iter()
            .rev()
            .copied()
            .find(|line| !self.maturity.is_young(&self.history.line_stats(*line)))
            .unwrap_or(newest);
        Some(BreakingTarget {
            to: self.history.newest_in(line, self.toolchain)?,
            line,
            young: self.maturity.is_young(&self.history.line_stats(line)),
            younger_lines: lines
                .iter()
                .copied()
                .filter(|younger| *younger > line)
                .collect(),
            lines_behind: u32::try_from(lines.len()).unwrap_or(u32::MAX),
        })
    }
}

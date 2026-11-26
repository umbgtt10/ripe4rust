// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::outcome::Outcome;
use crate::planning::judging::decision::Decision;
use crate::planning::judging::suggestion::Suggestion;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::traits::rule::Rule;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::vec;

/// Rule 9: a breaking or toolchain unit whose new line is young --
/// ripening, until it settles.
pub struct YoungLine;

impl Rule for YoungLine {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision> {
        let moving_line = matches!(facts.kind, UnitKind::Breaking | UnitKind::Toolchain);
        let line = facts.new_line.filter(|line| moving_line && line.young)?;
        let releases = Wording.count(line.stats.releases, "release");
        let reason = match line.stats.first_release {
            Some(first) => format!(
                "{} is young: {releases}, the first on {first}",
                line.stats.line
            ),
            None => format!("{} is young: {releases}", line.stats.line),
        };
        Some(Decision {
            outcome: Outcome::Ripening,
            urgent: false,
            rule: Some(9),
            suggestion: Some(Suggestion::UntilItSettles),
            reasons: vec![reason],
            why: None,
        })
    }
}

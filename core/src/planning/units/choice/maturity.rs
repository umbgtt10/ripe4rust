// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::units::choice::line_stats::LineStats;
use crate::values::date::Date;

/// Whether a line is young or settled, active or stale, as of today. A
/// release the index gives no date for is older than any it does: it makes
/// no line young and no line active.
pub struct Maturity {
    today: Date,
    young_days: u32,
    young_releases: u32,
    stale_months: u32,
}

impl Maturity {
    pub fn new(today: Date, young_days: u32, young_releases: u32, stale_months: u32) -> Self {
        Self {
            today,
            young_days,
            young_releases,
            stale_months,
        }
    }

    pub fn is_active(&self, stats: &LineStats) -> bool {
        let since = self.today.months_before(i64::from(self.stale_months));
        stats.last_release.is_some_and(|last| last >= since)
    }

    pub fn is_young(&self, stats: &LineStats) -> bool {
        let recent = stats
            .first_release
            .is_some_and(|first| self.today.days_since(&first) < i64::from(self.young_days));
        recent || stats.releases < self.young_releases
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::surveyed::Surveyed;
use crate::planning::units::choice::histories::Histories;
use crate::planning::units::choice::maturity::Maturity;
use crate::planning::units::choice::release_history::ReleaseHistory;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;

/// What every unit of a plan is judged against: the survey, the crates'
/// release histories and the toolchain's, and the maturity each is judged
/// by -- the toolchain's youth by `toolchain-young-days` alone, as a minor
/// rarely has three releases.
pub struct PlanContext<'a> {
    surveyed: &'a Surveyed,
    histories: &'a Histories,
    toolchain_history: &'a ReleaseHistory,
}

impl<'a> PlanContext<'a> {
    pub fn new(
        surveyed: &'a Surveyed,
        histories: &'a Histories,
        toolchain_history: &'a ReleaseHistory,
    ) -> Self {
        Self {
            surveyed,
            histories,
            toolchain_history,
        }
    }

    pub fn histories(&self) -> &Histories {
        self.histories
    }

    pub fn history_of(&self, unit: &Unit) -> Option<&ReleaseHistory> {
        if unit.kind == UnitKind::Toolchain {
            return Some(self.toolchain_history);
        }
        self.histories.get(unit.members.first()?)
    }

    pub fn maturity(&self) -> Maturity {
        let thresholds = self.surveyed.policy.thresholds;
        Maturity::new(
            self.surveyed.today,
            thresholds.young_days,
            thresholds.young_releases,
            thresholds.stale_months,
        )
    }

    pub fn maturity_of(&self, unit: &Unit) -> Maturity {
        let thresholds = self.surveyed.policy.thresholds;
        if unit.kind == UnitKind::Toolchain {
            Maturity::new(
                self.surveyed.today,
                thresholds.toolchain_young_days,
                0,
                thresholds.stale_months,
            )
        } else {
            self.maturity()
        }
    }

    pub fn surveyed(&self) -> &Surveyed {
        self.surveyed
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::judging::line_standing::LineStanding;
use crate::planning::units::choice::maturity::Maturity;
use crate::planning::units::choice::release_history::ReleaseHistory;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use crate::values::line::Line;
use alloc::vec::Vec;

/// A unit's lines: the one in use, the one it moves to -- young where any
/// member's is, a group's youth being its youngest member's --, and the
/// candidate lines newer than the one in use.
pub struct LineReview<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> LineReview<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn behind(&self, unit: &Unit) -> u32 {
        u32::try_from(self.newer_lines(unit).len()).unwrap_or(u32::MAX)
    }

    pub fn in_use(&self, unit: &Unit) -> Option<LineStanding> {
        let history = self.context.history_of(unit)?;
        let line = history.line_of(unit.from.as_ref()?);
        Some(Self::standing(
            history,
            line,
            &self.context.maturity_of(unit),
        ))
    }

    pub fn new_line(&self, unit: &Unit) -> Option<LineStanding> {
        if !matches!(unit.kind, UnitKind::Breaking | UnitKind::Toolchain) {
            return None;
        }
        let history = self.context.history_of(unit)?;
        let maturity = self.context.maturity_of(unit);
        let standing = Self::standing(history, history.line_of(unit.to.as_ref()?), &maturity);
        let young_member = unit.moves.iter().any(|lock_move| {
            let member = self.context.histories().get(&lock_move.name);
            member
                .zip(lock_move.to.as_ref())
                .is_some_and(|(member, to)| {
                    maturity.is_young(&member.line_stats(member.line_of(to)))
                })
        });
        Some(LineStanding {
            young: standing.young || young_member,
            ..standing
        })
    }

    pub fn newer_active(&self, unit: &Unit) -> bool {
        let maturity = self.context.maturity_of(unit);
        self.context.history_of(unit).is_some_and(|history| {
            self.newer_lines(unit)
                .into_iter()
                .any(|line| maturity.is_active(&history.line_stats(line)))
        })
    }

    fn newer_lines(&self, unit: &Unit) -> Vec<Line> {
        let toolchain = &self.context.surveyed().toolchain;
        match (self.context.history_of(unit), &unit.from) {
            (Some(history), Some(from)) => history.newer_lines(history.line_of(from), toolchain),
            _ => Vec::new(),
        }
    }

    fn standing(history: &ReleaseHistory, line: Line, maturity: &Maturity) -> LineStanding {
        let stats = history.line_stats(line);
        LineStanding {
            stats,
            young: maturity.is_young(&stats),
            active: maturity.is_active(&stats),
        }
    }
}

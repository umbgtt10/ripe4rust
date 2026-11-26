// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::judging::facts::downgrade_finder::DowngradeFinder;
use crate::planning::judging::facts::footprints::Footprints;
use crate::planning::judging::facts::held_back::HeldBack;
use crate::planning::judging::facts::line_review::LineReview;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::units::concern::Concern;
use crate::planning::units::own_move::OwnMove;
use crate::planning::units::unit::Unit;
use crate::planning::writing::evidence::Evidence;
use crate::unit_kind::UnitKind;
use alloc::vec::Vec;

/// A unit's evidence: what its decision read.
pub struct EvidenceWriter<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> EvidenceWriter<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn evidence(&self, unit: &Unit, concerns: &[Concern]) -> Evidence {
        let rust_version = unit
            .members
            .first()
            .zip(OwnMove.lands_on(unit))
            .filter(|_| unit.kind != UnitKind::Toolchain)
            .and_then(|(name, to)| self.context.histories().get(name)?.rust_version_of(to));
        let footprint = if unit.kind == UnitKind::Breaking {
            Footprints::new(self.context.surveyed().scan.as_ref()).locations(&unit.members)
        } else {
            Vec::new()
        };
        Evidence {
            line_in_use: LineReview::new(self.context)
                .in_use(unit)
                .map(|standing| standing.stats),
            rust_version,
            advisories: concerns
                .iter()
                .filter(|concern| matches!(concern, Concern::Advisory(_)))
                .map(Concern::label)
                .collect(),
            yanked_in_use: concerns
                .iter()
                .any(|concern| matches!(concern, Concern::Yanked { .. })),
            footprint,
            held_back: HeldBack::new(self.context).versions(unit),
            held: unit.held.clone(),
            downgrades: DowngradeFinder::new(self.context)
                .of(unit)
                .into_iter()
                .map(|downgrade| LockMove {
                    name: downgrade.name,
                    from: Some(downgrade.from),
                    to: Some(downgrade.to),
                })
                .collect(),
        }
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::copies::Copies;
use crate::planning::judging::facts::dependents::Dependents;
use crate::planning::judging::facts::downgrade_finder::DowngradeFinder;
use crate::planning::judging::facts::exposure::Exposure;
use crate::planning::judging::facts::footprints::Footprints;
use crate::planning::judging::facts::held_back::HeldBack;
use crate::planning::judging::facts::line_review::LineReview;
use crate::planning::judging::facts::pin_review::PinReview;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::judging::facts::reach::Reach;
use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::units::concern::Concern;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;

/// Everything the rules read of one unit, gathered from the plan's context.
/// A freeze is open while today is before `freeze-until`.
pub struct FactFinder<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> FactFinder<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn facts(&self, unit: &Unit, concerns: &[Concern]) -> UnitFacts {
        let surveyed = self.context.surveyed();
        let count = |items: usize| u32::try_from(items).unwrap_or(u32::MAX);
        let lines = LineReview::new(self.context);
        let pins = PinReview::new(self.context);
        let exposure = Exposure::new(self.context);
        let standings = pins.pins(unit);
        UnitFacts {
            kind: unit.kind,
            name: self.name(unit),
            rest: unit.name.is_none().then(|| count(unit.moves.len())),
            concerns: concerns.iter().map(Concern::label).collect(),
            downgrades: DowngradeFinder::new(self.context).of(unit),
            reaches: Reach::new(surveyed).targets(unit),
            fully_held: pins.fully_held(unit, &standings),
            pins: standings,
            freeze: surveyed
                .policy
                .freeze_until
                .filter(|until| surveyed.today < *until),
            exposed_in: exposure.exposed_in(unit),
            exposure_unknown_in: exposure.unknown_in(unit),
            raises: exposure.raises(unit),
            in_use_line: lines.in_use(unit),
            new_line: lines.new_line(unit),
            newer_active: lines.newer_active(unit),
            lines_behind: lines.behind(unit),
            footprint: Footprints::new(surveyed.scan.as_ref(), &surveyed.workspace.members)
                .footprint(&unit.members),
            removes_copy: Copies::new(&surveyed.workspace).removed(unit),
            held_by: Dependents::new(&surveyed.workspace).holding(unit),
            held_back: count(HeldBack::new(self.context).versions(unit).len()),
            gates: count(surveyed.policy.gates.len()),
            small_footprint: surveyed.policy.thresholds.small_footprint,
            moderate_footprint: surveyed.policy.thresholds.moderate_footprint,
        }
    }

    pub fn name(&self, unit: &Unit) -> String {
        match (unit.kind, &unit.name, &unit.from) {
            (UnitKind::Toolchain, ..) => String::from("the toolchain"),
            (UnitKind::Advisory, Some(name), Some(from)) => format!("{name} {from}"),
            (_, Some(name), _) => name.clone(),
            _ => String::from("the rest"),
        }
    }
}

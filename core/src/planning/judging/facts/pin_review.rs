// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::judging::pin_standing::PinStanding;
use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The pins that hold what a unit moves, and whether each is released. A
/// breaking unit's own pins hold it, on the line or on everything; a pin on
/// any crate of a git unit's source holds it whole, as a source has no line; the
/// toolchain's `rust` pin holds a new minor, or with `hold = "all"` every
/// move; a compatible unit's are the `all` pins on the moves held out of it.
pub struct PinReview<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> PinReview<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn fully_held(&self, unit: &Unit, pins: &[PinStanding]) -> bool {
        match unit.kind {
            UnitKind::Breaking | UnitKind::Git | UnitKind::Toolchain => !pins.is_empty(),
            UnitKind::Compatible => unit.moves.is_empty() && !unit.held.is_empty(),
            UnitKind::Advisory => false,
        }
    }

    pub fn pins(&self, unit: &Unit) -> Vec<PinStanding> {
        self.context
            .surveyed()
            .policy
            .pins
            .iter()
            .filter(|pin| Self::holds(pin, unit))
            .map(|pin| PinStanding {
                pin: pin.clone(),
                released: self.released(pin),
            })
            .collect()
    }

    pub fn released(&self, pin: &Pin) -> Option<String> {
        let released = match &pin.until {
            Some(ReleaseCondition::Release { name, requirement }) => self
                .context
                .histories()
                .get(name)
                .and_then(|history| history.newest_matching(requirement))
                .map(|version| format!("{name} {version} released")),
            _ => None,
        };
        let today = self.context.surveyed().today;
        released.or_else(|| {
            pin.review
                .filter(|review| *review <= today)
                .map(|review| format!("review {review} reached"))
        })
    }

    fn holds(pin: &Pin, unit: &Unit) -> bool {
        match unit.kind {
            UnitKind::Breaking | UnitKind::Git => unit.members.contains(&pin.name),
            UnitKind::Toolchain => {
                let new_minor = unit.from.as_ref().map(Version::toolchain_line)
                    != unit.to.as_ref().map(Version::toolchain_line);
                unit.members.contains(&pin.name) && (pin.hold == Hold::All || new_minor)
            }
            UnitKind::Compatible => {
                pin.hold == Hold::All && unit.held.iter().any(|held| held.name == pin.name)
            }
            UnitKind::Advisory => false,
        }
    }
}

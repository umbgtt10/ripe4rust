// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// What the toolchain unit would release: for each direct dependency, the
/// newest release newer than the one in use that the toolchain in use
/// cannot build and the new one can.
pub struct HeldBack<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> HeldBack<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn versions(&self, unit: &Unit) -> Vec<String> {
        let (UnitKind::Toolchain, Some(from), Some(to)) = (unit.kind, &unit.from, &unit.to) else {
            return Vec::new();
        };
        self.context
            .surveyed()
            .direct
            .iter()
            .filter_map(|dependency| {
                let newest = self
                    .context
                    .histories()
                    .get(&dependency.name)?
                    .held_back(from, &dependency.version, to)
                    .pop()?;
                Some(format!("{} {newest}", dependency.name))
            })
            .collect()
    }
}

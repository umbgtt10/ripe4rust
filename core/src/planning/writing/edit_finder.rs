// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::facts::exposure::Exposure;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::units::unit::Unit;
use crate::planning::writing::edits::Edits;
use crate::planning::writing::step::Step;
use crate::unit_kind::UnitKind;
use alloc::string::ToString;
use alloc::vec::Vec;

/// The edits a unit's `apply` makes: for a breaking unit, every line that
/// states a member's requirement and every member's `rust-version` its
/// targets raise; for the toolchain, the channel.
pub struct EditFinder<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> EditFinder<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn edits(&self, unit: &Unit) -> Vec<Step> {
        let surveyed = self.context.surveyed();
        match unit.kind {
            UnitKind::Breaking => {
                let requirements = unit.moves.iter().flat_map(|lock_move| {
                    let to = lock_move
                        .to
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    surveyed
                        .requirement_lines
                        .iter()
                        .filter(move |(name, _)| *name == lock_move.name)
                        .flat_map(move |(name, lines)| {
                            let to = to.clone();
                            lines
                                .iter()
                                .map(move |at| Edits.set(at.clone(), name, to.clone()))
                        })
                });
                let raises = Exposure::new(self.context).raises(unit);
                let rust_versions = raises.iter().filter_map(|raise| {
                    surveyed
                        .rust_version_lines
                        .iter()
                        .find(|(member, _)| *member == raise.member)
                        .map(|(_, at)| Edits.set(at.clone(), "rust-version", raise.to.to_string()))
                });
                let mut edits = requirements.collect::<Vec<Step>>();
                for edit in rust_versions {
                    if !edits.contains(&edit) {
                        edits.push(edit);
                    }
                }
                edits
            }
            UnitKind::Toolchain => surveyed
                .channel_at
                .clone()
                .zip(unit.to.as_ref())
                .map(|(at, to)| Edits.set(at, "channel", to.to_string()))
                .into_iter()
                .collect(),
            UnitKind::Compatible | UnitKind::Advisory | UnitKind::Git => Vec::new(),
        }
    }
}

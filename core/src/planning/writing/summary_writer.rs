// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::wording::Wording;
use crate::planning::units::concern::Concern;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

/// A unit's summary, the subject of the commit that takes it.
pub struct SummaryWriter;

impl SummaryWriter {
    pub fn summary(&self, unit: &Unit, concerns: &[Concern]) -> String {
        let lead = match unit.members.split_first() {
            Some((first, [])) => first.clone(),
            Some((first, others)) => format!("{first} with {}", others.join(", ")),
            None => String::new(),
        };
        let pulled_in = unit
            .moves
            .iter()
            .filter(|lock_move| !unit.members.contains(&lock_move.name))
            .count();
        let pulled = Wording.count(u32::try_from(pulled_in).unwrap_or(u32::MAX), "crate");
        let to = unit
            .to
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        match unit.kind {
            UnitKind::Advisory => format!(
                "Resolve {} in {lead}{}",
                Wording.list(&concerns.iter().map(Concern::label).collect::<Vec<String>>()),
                unit.name
                    .as_ref()
                    .zip(unit.from.as_ref())
                    .map(|(name, from)| format!("{name} {from}"))
                    .unwrap_or_default()
            ),
            UnitKind::Breaking => format!("Migrate {lead} to {to}"),
            UnitKind::Toolchain => format!("Move the toolchain to {to}"),
            UnitKind::Compatible if unit.name.is_none() => format!("Move {pulled} no unit reaches"),
            UnitKind::Compatible if unit.to.is_none() || unit.from == unit.to => {
                format!("Move {pulled} {lead} pulls in")
            }
            UnitKind::Compatible if pulled_in == 0 => format!("Move {lead} to {to}"),
            UnitKind::Compatible => format!("Move {lead} to {to}, with {pulled} it pulls in"),
        }
    }
}

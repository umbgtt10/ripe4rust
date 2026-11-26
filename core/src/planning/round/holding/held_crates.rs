// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::round::holding::held_crate::HeldCrate;
use crate::planning::round::target::Target;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const RUST: &str = "rust";

/// The crates a unit's pins hold, so the next plan holds the unit: a
/// breaking unit's crate on its line, which holds a group by its first
/// member; every crate a compatible unit or a split move moves, whole, since a
/// pin otherwise holds only a new line; the toolchain as `rust`, on its line
/// for a new minor and whole for a patch release; a git unit by its crate,
/// whole, as a pin on any crate of its source holds it.
pub struct HeldCrates;

impl HeldCrates {
    pub fn of(&self, target: &Target) -> Vec<HeldCrate> {
        let unit = &target.unit;
        match unit.kind {
            UnitKind::Breaking => unit
                .name
                .iter()
                .map(|name| HeldCrate {
                    name: name.clone(),
                    hold: Hold::Line,
                })
                .collect(),
            UnitKind::Compatible => {
                let mut names = target
                    .moves
                    .iter()
                    .map(|lock_move| lock_move.name.clone())
                    .collect::<Vec<String>>();
                names.dedup();
                names
                    .into_iter()
                    .map(|name| HeldCrate {
                        name,
                        hold: Hold::All,
                    })
                    .collect()
            }
            UnitKind::Toolchain => {
                let line = |version: Option<&Version>| version.map(Version::toolchain_line);
                let hold = if line(unit.from.as_ref()) == line(unit.to.as_ref()) {
                    Hold::All
                } else {
                    Hold::Line
                };
                vec![HeldCrate {
                    name: String::from(RUST),
                    hold,
                }]
            }
            UnitKind::Git => unit
                .name
                .iter()
                .map(|name| HeldCrate {
                    name: name.clone(),
                    hold: Hold::All,
                })
                .collect(),
            UnitKind::Advisory => Vec::new(),
        }
    }
}

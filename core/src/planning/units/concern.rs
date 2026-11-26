// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::advisory::Advisory;
use crate::inputs::lock_move::LockMove;
use crate::planning::units::unit::Unit;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// What makes a unit urgent: an advisory against a version in use, or a
/// yanked release in use, which is treated as one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Concern {
    Advisory(Advisory),
    Yanked { name: String, version: Version },
}

impl Concern {
    pub fn ignore_entry(&self) -> String {
        match self {
            Self::Advisory(advisory) => format!(
                "{{ id = \"{}\", reason = \"<the person's words>\" }}",
                advisory.id
            ),
            Self::Yanked { name, version } => {
                format!("{{ crate = \"{name}@{version}\", reason = \"<the person's words>\" }}")
            }
        }
    }

    pub fn is_cleared_by(&self, lock_move: &LockMove) -> bool {
        let Some(to) = &lock_move.to else {
            return false;
        };
        match self {
            Self::Advisory(advisory) => {
                lock_move.name == advisory.name
                    && lock_move.from.as_ref() == Some(&advisory.version)
                    && advisory
                        .patched
                        .iter()
                        .chain(&advisory.unaffected)
                        .any(|clear| clear.matches(to))
            }
            Self::Yanked { name, version } => {
                lock_move.name == *name && lock_move.from.as_ref() == Some(version) && to != version
            }
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::Advisory(advisory) => advisory.id.clone(),
            Self::Yanked { name, version } => format!("the yanked {name} {version}"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Advisory(advisory) => &advisory.name,
            Self::Yanked { name, .. } => name,
        }
    }

    pub fn row(&self) -> Unit {
        let (name, version) = match self {
            Self::Advisory(advisory) => (&advisory.name, &advisory.version),
            Self::Yanked { name, version } => (name, version),
        };
        Unit {
            id: UnitId::of(UnitKind::Advisory, &format!("{name}@{version}")),
            kind: UnitKind::Advisory,
            name: Some(name.clone()),
            members: Vec::new(),
            from: Some(version.clone()),
            to: None,
            moves: Vec::new(),
            held: Vec::new(),
            git: None,
        }
    }
}

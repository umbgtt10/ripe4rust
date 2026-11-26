// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::choice::release_history::ReleaseHistory;
use crate::planning::units::unit::Unit;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const RUST: &str = "rust";

/// The toolchain unit: `rust-toolchain.toml`'s channel to the newest stable
/// release, where the channel pins an older one.
pub struct ToolchainUnit<'a> {
    history: &'a ReleaseHistory,
}

impl<'a> ToolchainUnit<'a> {
    pub fn new(history: &'a ReleaseHistory) -> Self {
        Self { history }
    }

    pub fn build(&self, pinned: Option<&Version>) -> Option<Unit> {
        let pinned = pinned?;
        let newest = self.history.candidates(pinned).pop()?;
        (newest > *pinned).then(|| Unit {
            id: UnitId::of(UnitKind::Toolchain, RUST),
            kind: UnitKind::Toolchain,
            name: Some(String::from(RUST)),
            members: vec![String::from(RUST)],
            from: Some(pinned.clone()),
            to: Some(newest),
            moves: Vec::new(),
            held: Vec::new(),
        })
    }
}

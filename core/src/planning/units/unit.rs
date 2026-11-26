// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::unit_id::UnitId;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// The smallest upgrade worth one commit, before it is judged: the crates it
/// moves directly (a group's members, `rust` for the toolchain), the lock
/// moves it applies, and those an `all` pin holds out of it. `name` is
/// `None` for the rest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit {
    pub id: UnitId,
    pub kind: UnitKind,
    pub name: Option<String>,
    pub members: Vec<String>,
    pub from: Option<Version>,
    pub to: Option<Version>,
    pub moves: Vec<LockMove>,
    pub held: Vec<LockMove>,
}

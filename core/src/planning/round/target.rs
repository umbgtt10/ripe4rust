// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::inputs::recorded_unit::RecordedUnit;
use crate::inputs::written_line::WrittenLine;
use alloc::string::String;
use alloc::vec::Vec;

/// What a round's command is about: the id it was given, the unit of the
/// plan that id names, and what applying it moves and edits -- for one move
/// of a unit's split, that move alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Target {
    pub id: String,
    pub unit: RecordedUnit,
    pub moves: Vec<LockMove>,
    pub edits: Vec<WrittenLine>,
}

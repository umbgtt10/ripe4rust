// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::inputs::written_line::WrittenLine;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// One row of a plan as `plan` wrote it, what the round's commands read of
/// it: its id and kind, the crates it moves directly, its versions, the lock
/// moves and the edits its `apply` makes, whether it is urgent, the ids of its
/// split, and the command that reverts it -- `None` for an advisory row, which
/// has no steps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordedUnit {
    pub id: String,
    pub kind: UnitKind,
    pub name: Option<String>,
    pub members: Vec<String>,
    pub from: Option<Version>,
    pub to: Option<Version>,
    pub moves: Vec<LockMove>,
    pub urgent: bool,
    pub edits: Vec<WrittenLine>,
    pub splits: Vec<String>,
    pub revert: Option<String>,
}

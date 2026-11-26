// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

/// A change of the lock no unit of the plan makes, in words.
pub struct MoveWording;

impl MoveWording {
    pub fn unexplained(&self, lock_move: &LockMove) -> String {
        let name = &lock_move.name;
        match (&lock_move.from, &lock_move.to) {
            (Some(from), Some(to)) => {
                format!("`{name}` moved {from} -> {to}, a move no unit of the plan makes")
            }
            (None, Some(to)) => {
                format!("`{name}` {to} joined the lock, which no unit of the plan adds")
            }
            (from, None) => format!(
                "`{name}` {} left the lock, which no unit of the plan removes",
                from.as_ref().map(ToString::to_string).unwrap_or_default()
            ),
        }
    }
}

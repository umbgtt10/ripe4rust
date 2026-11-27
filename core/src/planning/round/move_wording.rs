// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_source::GitSource;
use crate::inputs::lock_move::LockMove;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

const SHARED: &str =
    "the units share the crate, and its moves depend on the order they are applied in";

/// A change of the lock no unit of the plan makes, in words: a crate's, or a
/// git source's from one rev to another; and one of a crate the units share,
/// said to be no difference and why.
pub struct MoveWording;

impl MoveWording {
    pub fn git(&self, moved: &GitSource) -> String {
        format!(
            "`{}` moved {} -> {}, a move no unit of the plan makes",
            moved.source, moved.from, moved.to
        )
    }

    pub fn shared(&self, lock_move: &LockMove) -> String {
        let name = &lock_move.name;
        let moved = match (&lock_move.from, &lock_move.to) {
            (Some(from), Some(to)) => format!("`{name}` moved {from} -> {to}"),
            (None, Some(to)) => format!("`{name}` {to} joined the lock"),
            (from, None) => format!(
                "`{name}` {} left the lock",
                from.as_ref().map(ToString::to_string).unwrap_or_default()
            ),
        };
        self.why_shared(&moved)
    }

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

    pub fn why_shared(&self, what: &str) -> String {
        format!("{what}: {SHARED}")
    }
}

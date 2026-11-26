// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::move_document::MoveDocument;
use ripe4rust_core::inputs::lock_move::LockMove;

/// A lock move: the crate, and the versions it leaves and takes, `null` for
/// a crate added or removed.
pub struct MoveJson;

impl MoveJson {
    pub fn document<'a>(&self, lock_move: &'a LockMove) -> MoveDocument<'a> {
        MoveDocument {
            name: &lock_move.name,
            from: lock_move.from.as_ref().map(ToString::to_string),
            to: lock_move.to.as_ref().map(ToString::to_string),
        }
    }
}

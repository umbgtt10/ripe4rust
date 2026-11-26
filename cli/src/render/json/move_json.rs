// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::lock_move::LockMove;
use serde_json::Value;
use serde_json::json;

/// A lock move: the crate, and the versions it leaves and takes, `null` for
/// a crate added or removed.
pub struct MoveJson;

impl MoveJson {
    pub fn value(&self, lock_move: &LockMove) -> Value {
        json!({
            "crate": lock_move.name,
            "from": lock_move.from.as_ref().map(ToString::to_string),
            "to": lock_move.to.as_ref().map(ToString::to_string),
        })
    }
}

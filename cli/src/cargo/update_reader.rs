// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::cargo::move_line::MoveLine;
use crate::cargo::status_line::StatusLine;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;

/// What `cargo update --dry-run` printed, read strictly: every line a move
/// or a status it knows, and the first it does not know stops the plan,
/// rather than leave a move out of it.
pub struct UpdateReader;

impl UpdateReader {
    pub fn moves(&self, text: &str) -> Result<Vec<LockMove>, PortFailure> {
        let mut moves = Vec::new();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            match MoveLine.read(line) {
                Some(found) => moves.push(found),
                None if StatusLine.knows(line) => {}
                None => {
                    return Err(PortFailure::CargoUpdateUnrecognised {
                        line: String::from(line),
                    });
                }
            }
        }
        Ok(moves)
    }
}

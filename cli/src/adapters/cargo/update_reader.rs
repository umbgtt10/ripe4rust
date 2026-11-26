// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::git_move_line::GitMoveLine;
use crate::adapters::cargo::move_line::MoveLine;
use crate::adapters::cargo::status_line::StatusLine;
use ripe4rust_core::inputs::git::moves::Moves;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;

/// What `cargo update --dry-run` printed, read strictly: every line a move
/// -- a registry crate's, or a git source's crate's, read apart -- or a status
/// it knows, and the first it does not know stops the plan, rather than leave
/// a move out of it. `moves` is the registry's alone.
pub struct UpdateReader;

impl UpdateReader {
    pub fn moves(&self, text: &str) -> Result<Vec<LockMove>, PortFailure> {
        Ok(self.read(text)?.registry)
    }

    pub fn read(&self, text: &str) -> Result<Moves, PortFailure> {
        let mut moves = Moves::default();
        for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            if let Some(found) = MoveLine.read(line) {
                moves.registry.push(found);
            } else if let Some(found) = GitMoveLine.read(line) {
                moves.git.push(found);
            } else if !StatusLine.knows(line) {
                return Err(PortFailure::CargoUpdateUnrecognised {
                    line: String::from(line),
                });
            }
        }
        Ok(moves)
    }
}

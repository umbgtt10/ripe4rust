// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::git::moves::Moves;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::lock_moves::LockMoves;
use ripe4rust_core::values::version::Version;

pub struct FakeLockMoves {
    pub of_crate: Vec<(String, Vec<LockMove>)>,
    pub of_source: Vec<(String, Moves)>,
    pub of_workspace: Moves,
    pub failure: Option<PortFailure>,
}

impl LockMoves for FakeLockMoves {
    fn of_crate(&self, name: &str, _version: &Version) -> Result<Vec<LockMove>, PortFailure> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        Ok(self
            .of_crate
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, moves)| moves.clone())
            .unwrap_or_default())
    }

    fn of_source(&self, name: &str, _version: &Version) -> Result<Moves, PortFailure> {
        Ok(self
            .of_source
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, moves)| moves.clone())
            .unwrap_or_default())
    }

    fn of_workspace(&self) -> Result<Moves, PortFailure> {
        Ok(self.of_workspace.clone())
    }
}

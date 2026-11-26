// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::cargo_command::CargoCommand;
use crate::adapters::cargo::update_reader::UpdateReader;
use crate::system::traits::command_runner::CommandRunner;
use ripe4rust_core::inputs::git::moves::Moves;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::lock_moves::LockMoves;
use ripe4rust_core::values::version::Version;
use std::path::PathBuf;

/// `cargo update --dry-run`, which writes nothing: for one direct
/// dependency at its version with `--recursive -p <crate>@<version>`, for a
/// git source by one of its crates with `-p <crate>@<version>`, which moves
/// the source as a whole, and for the whole workspace; `--offline` in an offline plan. What it prints
/// is read strictly.
pub struct CargoUpdate<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
    offline: bool,
}

impl<'a> CargoUpdate<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf, offline: bool) -> Self {
        Self {
            runner,
            dir,
            offline,
        }
    }

    fn moves(&self, package: &[String]) -> Result<Moves, PortFailure> {
        let mut args = ["update", "--dry-run"].map(String::from).to_vec();
        if self.offline {
            args.push(String::from("--offline"));
        }
        args.extend(package.iter().cloned());
        let output = CargoCommand::new(self.runner, self.dir.clone())
            .run(args, |detail| PortFailure::CargoUpdateFailed { detail })?;
        UpdateReader.read(&output.stderr)
    }
}

impl LockMoves for CargoUpdate<'_> {
    fn of_crate(&self, name: &str, version: &Version) -> Result<Vec<LockMove>, PortFailure> {
        Ok(self
            .moves(&[
                String::from("--recursive"),
                String::from("-p"),
                format!("{name}@{version}"),
            ])?
            .registry)
    }

    fn of_source(&self, name: &str, version: &Version) -> Result<Moves, PortFailure> {
        self.moves(&[String::from("-p"), format!("{name}@{version}")])
    }

    fn of_workspace(&self) -> Result<Moves, PortFailure> {
        self.moves(&[])
    }
}

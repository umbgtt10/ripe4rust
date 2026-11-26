// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::cargo_command::CargoCommand;
use crate::adapters::workspace::layout_reader::LayoutReader;
use crate::system::model::layout::Layout;
use crate::system::traits::command_runner::CommandRunner;
use crate::system::traits::layout_source::LayoutSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::cell::OnceCell;
use std::path::PathBuf;

/// The layout as `cargo metadata --format-version 1 --no-deps` gives it,
/// asked once, from wherever the command runs: it resolves nothing, so it
/// needs no network and moves no lock.
pub struct CargoLayout<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
    layout: OnceCell<Result<Layout, PortFailure>>,
}

impl<'a> CargoLayout<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf) -> Self {
        Self {
            runner,
            dir,
            layout: OnceCell::new(),
        }
    }
}

impl LayoutSource for CargoLayout<'_> {
    fn layout(&self) -> Result<Layout, PortFailure> {
        self.layout
            .get_or_init(|| {
                let args = ["metadata", "--format-version", "1", "--no-deps"]
                    .map(String::from)
                    .to_vec();
                let output = CargoCommand::new(self.runner, self.dir.clone())
                    .run(args, |detail| PortFailure::CargoMetadataFailed { detail })?;
                LayoutReader.layout(&output.stdout)
            })
            .clone()
    }
}

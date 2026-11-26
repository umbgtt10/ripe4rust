// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::traits::command_runner::CommandRunner;
use crate::system::traits::file_system::FileSystem;
use crate::system::traits::pause::Pause;
use ripe4rust_core::traits::clock::Clock;
use std::path::PathBuf;

/// Everything a run reaches the system through, each a seam `main` fills
/// with the real one: the processes, the files, the pause between API
/// requests, the clock; the directory it runs in, and where the day cache
/// lives, if anywhere.
#[derive(Clone)]
pub struct Seams<'a> {
    pub runner: &'a dyn CommandRunner,
    pub files: &'a dyn FileSystem,
    pub pause: &'a dyn Pause,
    pub clock: &'a dyn Clock,
    pub dir: PathBuf,
    pub cache: Option<PathBuf>,
}

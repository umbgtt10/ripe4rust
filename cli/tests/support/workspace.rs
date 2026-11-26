// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::outputs::printed;
use crate::support::replay_runner::ReplayRunner;

pub const LAYOUT: [&str; 5] = ["cargo", "metadata", "--format-version", "1", "--no-deps"];

/// The recorded workspace's files, at the paths cargo reported them.
pub fn files() -> MemoryFiles {
    MemoryFiles::new()
        .with(
            "/work/ws/Cargo.toml",
            include_str!("../fixtures/workspace/files/Cargo.toml.txt"),
        )
        .with(
            "/work/ws/Cargo.lock",
            include_str!("../fixtures/workspace/files/Cargo.lock.txt"),
        )
        .with(
            "/work/ws/rust-toolchain.toml",
            include_str!("../fixtures/workspace/files/rust-toolchain.toml.txt"),
        )
        .with(
            "/work/ws/app/Cargo.toml",
            include_str!("../fixtures/workspace/files/app/Cargo.toml.txt"),
        )
        .with(
            "/work/ws/app-core/Cargo.toml",
            include_str!("../fixtures/workspace/files/app-core/Cargo.toml.txt"),
        )
}

/// `cargo metadata --no-deps` as recorded for the workspace.
pub fn layout() -> ReplayRunner {
    ReplayRunner::new().reply(
        &LAYOUT,
        printed(
            include_str!("../fixtures/workspace/cargo/metadata-no-deps.json"),
            "",
        ),
    )
}

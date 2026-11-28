// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fakes::fake_advisories::FakeAdvisories;
use crate::support::fakes::fake_clock::FakeClock;
use crate::support::fakes::fake_files::FakeFiles;
use crate::support::fakes::fake_index::FakeIndex;
use crate::support::fakes::fake_lock_moves::FakeLockMoves;
use crate::support::fakes::fake_metadata::FakeMetadata;
use crate::support::fakes::fake_repositories::FakeRepositories;
use crate::support::fakes::fake_scan::FakeScan;
use crate::support::fakes::fake_toolchain::FakeToolchain;
use crate::support::policy_trees::entry;
use crate::support::policy_trees::texts;
use crate::support::policy_trees::tree;
use crate::support::surveyed::at;
use crate::support::surveyed::surveyed;
use ripe4rust_core::inputs::git::moves::Moves;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::planning::ports::Ports;
use std::cell::RefCell;

/// Every port faked, answering what the `surveyed` fixture holds.
pub struct FakePorts {
    pub clock: FakeClock,
    pub index: FakeIndex,
    pub repositories: FakeRepositories,
    pub lock_moves: FakeLockMoves,
    pub metadata: FakeMetadata,
    pub files: FakeFiles,
    pub advisories: FakeAdvisories,
    pub toolchain: FakeToolchain,
    pub scan: FakeScan,
}

impl FakePorts {
    pub fn new() -> Self {
        let surveyed = surveyed();
        Self {
            clock: FakeClock {
                today: surveyed.today,
            },
            index: FakeIndex {
                read_on: surveyed.registry_read,
                releases: surveyed.releases.clone(),
                failure: None,
                asked: RefCell::new(Vec::new()),
            },
            repositories: FakeRepositories { known: Vec::new() },
            lock_moves: FakeLockMoves {
                of_crate: surveyed
                    .dependency_moves
                    .iter()
                    .map(|each| (each.dependency.name.clone(), each.moves.clone()))
                    .collect(),
                of_source: Vec::new(),
                of_workspace: Moves {
                    registry: surveyed.workspace_moves.clone(),
                    git: Vec::new(),
                },
                failure: None,
            },
            metadata: FakeMetadata {
                workspace: surveyed.workspace.clone(),
                reached: Vec::new(),
                failure: None,
            },
            files: FakeFiles {
                fingerprint: surveyed.fingerprint.clone(),
                lock: Vec::new(),
                policy: tree(vec![
                    entry("gates", texts(&["just stage1", "just stage2"]), 31),
                    entry("published", texts(&["app-core"]), 32),
                ]),
                toolchain: Some(ToolchainFile {
                    channel: String::from("1.99.1"),
                    at: at("rust-toolchain.toml", 9),
                }),
                requirement_lines: surveyed.requirement_lines.clone(),
                rust_version_lines: surveyed.rust_version_lines.clone(),
                written: surveyed.written.clone(),
                git: surveyed.git.clone(),
                edges: surveyed.edges.clone(),
                failure: None,
            },
            advisories: FakeAdvisories {
                advisories: Vec::new(),
                failure: None,
            },
            toolchain: FakeToolchain {
                releases: Ok(surveyed.toolchain_releases.clone()),
                rustc: surveyed.toolchain.clone(),
                targets: vec![String::from("thumbv7em-none-eabihf")],
            },
            scan: FakeScan {
                scan: surveyed.scan.clone(),
            },
        }
    }

    pub fn ports(&self) -> Ports<'_> {
        Ports {
            clock: &self.clock,
            index: &self.index,
            repositories: &self.repositories,
            lock_moves: &self.lock_moves,
            metadata: &self.metadata,
            files: &self.files,
            advisories: &self.advisories,
            toolchain: &self.toolchain,
            scan: &self.scan,
        }
    }
}

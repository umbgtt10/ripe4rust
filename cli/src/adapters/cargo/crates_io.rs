// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const SOURCES: [&str; 2] = [
    "registry+https://github.com/rust-lang/crates.io-index",
    "sparse+https://index.crates.io/",
];

/// Whether a package comes from crates.io, the one registry in scope: path
/// and git packages, and other registries', are the workspace's own
/// business.
pub struct CratesIo;

impl CratesIo {
    pub fn holds(&self, source: Option<&str>) -> bool {
        source.is_some_and(|source| SOURCES.contains(&source))
    }
}

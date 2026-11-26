// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The kinds of row a plan holds: the four kinds of upgrade, each one
/// commit's worth, and the row of an advisory no upgrade clears.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    /// An advisory or a yanked release no unit of the plan clears.
    Advisory,
    /// One direct dependency to a new major (for `0.x`, a new minor).
    Breaking,
    /// The semver-compatible lock-file moves one direct dependency pulls in,
    /// or, for the rest, those no unit reaches.
    Compatible,
    /// A git source moved to a new rev, every crate of it at once, with what
    /// it pulls in.
    Git,
    /// `rust-toolchain.toml`'s channel to a newer stable release.
    Toolchain,
}

impl UnitKind {
    pub fn parse(label: &str) -> Option<Self> {
        [
            Self::Advisory,
            Self::Breaking,
            Self::Compatible,
            Self::Git,
            Self::Toolchain,
        ]
        .into_iter()
        .find(|kind| kind.label() == label)
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Breaking => "breaking",
            Self::Compatible => "compatible",
            Self::Git => "git",
            Self::Toolchain => "toolchain",
        }
    }
}

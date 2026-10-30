// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The two kinds of upgrade, each one commit's worth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitKind {
    /// One direct dependency to a new major (for `0.x`, a new minor), with what
    /// the lock file moves along with it.
    Breaking,
    /// The semver-compatible lock-file moves one direct dependency pulls in.
    Compatible,
}

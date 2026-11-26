// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::version::Version;

/// An advisory cargo-deny reports against a crate at a version in use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdvisoryHit {
    pub id: String,
    pub name: String,
    pub version: Version,
}

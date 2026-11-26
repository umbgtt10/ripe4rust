// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

/// The workspace a plan was made from: the fingerprint of its lock, its
/// manifests and `rust-toolchain.toml`, and the lock's packages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceState {
    pub fingerprint: String,
    pub lock: Vec<String>,
}

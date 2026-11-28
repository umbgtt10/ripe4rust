// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::written_line::WrittenLine;
use alloc::string::String;
use alloc::vec::Vec;

/// The workspace a plan was made from: the fingerprint of its lock, its
/// manifests and `rust-toolchain.toml`, the lock's packages, every line a
/// unit's `apply` can edit, as written then, every crate of a git source
/// at its rev, `<crate> <source>#<rev>`, and every dependency the lock writes
/// with a version, `<crate> <version> -> <dependency> <version>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspaceState {
    pub fingerprint: String,
    pub lock: Vec<String>,
    pub lines: Vec<WrittenLine>,
    pub git: Vec<String>,
    pub edges: Vec<String>,
}

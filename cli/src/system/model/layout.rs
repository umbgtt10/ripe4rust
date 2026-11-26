// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

/// Where the workspace's files are: its root, and each member's manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    pub root: PathBuf,
    pub members: Vec<(String, PathBuf)>,
}

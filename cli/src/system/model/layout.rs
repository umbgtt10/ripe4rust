// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::member_sources::MemberSources;
use std::path::PathBuf;

/// Where the workspace's files are: its root, each member's manifest, and
/// each member's sources -- its targets and the dependencies they can name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    pub root: PathBuf,
    pub members: Vec<(String, PathBuf)>,
    pub sources: Vec<MemberSources>,
}

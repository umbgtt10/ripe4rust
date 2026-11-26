// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// How much of the workspace names a crate: lines, and the files they are in;
/// not `complete` where a member using it includes code the parse could not
/// follow, so the count is a floor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Footprint {
    pub lines: u32,
    pub files: u32,
    pub complete: bool,
}

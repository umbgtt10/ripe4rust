// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// One name a `use` item brings in: the first name of its path and that
/// name's line, whether the path starts with `::`, the name it binds --
/// `None` for a glob or `as _` --, its own line, and whether it is a glob.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UseLeaf {
    pub root: String,
    pub root_line: u32,
    pub global: bool,
    pub bound: Option<String>,
    pub line: u32,
    pub glob: bool,
}

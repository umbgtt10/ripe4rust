// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

/// A target of a member as `cargo metadata` lists it: its kinds -- `lib`,
/// `bin`, `test`, `bench`, `example`, `custom-build` and the rest -- and the
/// root file its modules are declared from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceTarget {
    pub kinds: Vec<String>,
    pub root: PathBuf,
}

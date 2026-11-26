// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::location::Location;
use std::path::PathBuf;

/// Where a module's file may be, in the order to try: a target's root, or
/// the files a `mod` item can name; whether the file found owns its
/// directory as `mod.rs` does -- a root, or one a `#[path]` names --;
/// whether it is reachable through `pub mod` declarations; and the `mod` item
/// declaring it, `None` for a root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSource {
    pub candidates: Vec<PathBuf>,
    pub owns_directory: bool,
    pub public: bool,
    pub declared_at: Option<Location>,
}

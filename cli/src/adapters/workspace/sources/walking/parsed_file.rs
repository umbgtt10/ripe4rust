// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
use syn::File;

/// A source file the parse read: where it is, its name from the workspace's
/// root, its syntax, and whether it is reachable from its library's root
/// through `pub mod` declarations.
#[derive(Clone)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub name: String,
    pub syntax: File,
    pub public: bool,
}

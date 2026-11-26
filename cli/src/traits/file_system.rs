// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::seams::file_failure::FileFailure;
use std::path::Path;

/// The files ripe4rust reads and the cache it writes: a file's text, a
/// directory's entries, and a file written whole, its directories made.
pub trait FileSystem {
    fn entries(&self, directory: &Path) -> Result<Vec<String>, FileFailure>;
    fn read(&self, path: &Path) -> Result<String, FileFailure>;
    fn write(&self, path: &Path, contents: &str) -> Result<(), FileFailure>;
}

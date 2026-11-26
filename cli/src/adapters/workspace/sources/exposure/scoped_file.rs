// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::walking::parsed_file::ParsedFile;

/// A file the parse read, and what its names stand for.
#[derive(Clone, Copy)]
pub struct ScopedFile<'a> {
    pub file: &'a ParsedFile,
    pub scope: &'a ImportScope<'a>,
}

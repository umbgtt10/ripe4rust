// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::walking::parsed_file::ParsedFile;
use ripe4rust_core::inputs::unparsed_file::UnparsedFile;

/// What walking a target's modules found: every file it read, in the order
/// of their names, and every one it could not.
#[derive(Clone)]
pub struct Walk {
    pub parsed: Vec<ParsedFile>,
    pub unparsed: Vec<UnparsedFile>,
}

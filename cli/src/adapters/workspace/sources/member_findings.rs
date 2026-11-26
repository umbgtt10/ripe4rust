// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::unparsed_file::UnparsedFile;
use ripe4rust_core::values::location::Location;

/// What the parse found of one member, or of one of its targets: every line
/// naming a dependency, by package; the dependencies in its library's public
/// API; the files it could not read; whether its library's API was not read
/// in full; how many files it read; and where it includes code the parse
/// could not follow.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MemberFindings {
    pub named: Vec<(String, Location)>,
    pub exposed: Vec<String>,
    pub unparsed: Vec<UnparsedFile>,
    pub api_unread: bool,
    pub parsed: usize,
    pub unfollowed: Vec<Location>,
}

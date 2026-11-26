// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A file an `include!` of a literal path pulls in: the path as written,
/// from the including file's directory, the line of the `include!`, whether
/// its items are reachable through `pub mod` declarations, and the module they
/// land in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncludedFile {
    pub path: String,
    pub line: u32,
    pub public: bool,
    pub module: Option<String>,
}

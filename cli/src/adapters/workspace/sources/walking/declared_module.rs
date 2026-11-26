// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A module a file declares out of line, `mod name;`: its name, the inline
/// modules it sits in, the path a `#[path]` gives it, whether it is reachable
/// from the library's root through `pub mod` declarations, and the line it is
/// declared on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredModule {
    pub name: String,
    pub inline: Vec<String>,
    pub path: Option<String>,
    pub public: bool,
    pub line: u32,
}

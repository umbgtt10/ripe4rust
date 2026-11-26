// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// An `extern crate` item: the crate as the code names it, the name it binds
/// where `as` renames it, and whether `#[macro_use]` brings its macros into
/// the whole target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternDeclaration {
    pub name: String,
    pub alias: Option<String>,
    pub macro_use: bool,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::extern_declaration::ExternDeclaration;
use crate::adapters::workspace::sources::naming::use_leaf::UseLeaf;

/// What a file declares that its names resolve through: the names its `use`
/// items bring in, its `extern crate` items, the names of its own items,
/// variants and generic parameters, and the macros it defines with
/// `macro_rules!`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Declarations {
    pub uses: Vec<UseLeaf>,
    pub externs: Vec<ExternDeclaration>,
    pub locals: Vec<String>,
    pub macros: Vec<String>,
}

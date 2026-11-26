// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use syn::Item;

/// An item of a library, whether its module is reachable from the library's
/// root through `pub mod` declarations, the name of the module holding it --
/// `None` at a target's root -- and the scope of its file.
#[derive(Clone)]
pub struct PlacedItem<'a> {
    pub item: &'a Item,
    pub public: bool,
    pub module: Option<String>,
    pub scope: &'a ImportScope<'a>,
}

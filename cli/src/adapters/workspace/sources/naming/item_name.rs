// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Ident;
use syn::Item;

/// The name an item declares, where it declares one: a constant, an enum, a
/// function, a module, a static, a struct, a trait or a trait alias, a type
/// alias, a union.
pub struct ItemName;

impl ItemName {
    pub fn of<'a>(&self, item: &'a Item) -> Option<&'a Ident> {
        match item {
            Item::Const(item) => Some(&item.ident),
            Item::Enum(item) => Some(&item.ident),
            Item::Fn(item) => Some(&item.sig.ident),
            Item::Mod(item) => Some(&item.ident),
            Item::Static(item) => Some(&item.ident),
            Item::Struct(item) => Some(&item.ident),
            Item::Trait(item) => Some(&item.ident),
            Item::TraitAlias(item) => Some(&item.ident),
            Item::Type(item) => Some(&item.ident),
            Item::Union(item) => Some(&item.ident),
            _ => None,
        }
    }
}

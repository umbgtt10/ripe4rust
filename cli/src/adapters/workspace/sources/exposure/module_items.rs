// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::placed_item::PlacedItem;
use crate::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use syn::Item;
use syn::ItemMod;
use syn::Visibility;

/// Every item of a library's files, inline modules' too, each with whether
/// its module is reachable from the root through `pub mod` declarations: its
/// file is, and every inline module on the way is `pub`.
pub struct ModuleItems;

impl ModuleItems {
    pub fn items<'a>(
        items: &'a [Item],
        public: bool,
        scope: &'a ImportScope<'a>,
    ) -> Vec<PlacedItem<'a>> {
        items
            .iter()
            .flat_map(|item| {
                let inner = match item {
                    Item::Mod(ItemMod {
                        vis,
                        content: Some((_, inner)),
                        ..
                    }) => Self::items(inner, public && matches!(vis, Visibility::Public(_)), scope),
                    _ => Vec::new(),
                };
                [PlacedItem {
                    item,
                    public,
                    scope,
                }]
                .into_iter()
                .chain(inner)
            })
            .collect()
    }

    pub fn of<'a>(&self, files: &[ScopedFile<'a>]) -> Vec<PlacedItem<'a>> {
        files
            .iter()
            .flat_map(|scoped| {
                Self::items(&scoped.file.syntax.items, scoped.file.public, scoped.scope)
            })
            .collect()
    }
}

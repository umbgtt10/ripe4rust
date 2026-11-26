// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::placed_item::PlacedItem;
use crate::adapters::workspace::sources::exposure::re_exports::ReExports;
use crate::adapters::workspace::sources::naming::item_name::ItemName;
use crate::adapters::workspace::sources::naming::use_leaves::UseLeaves;
use std::collections::BTreeSet;
use syn::Item;
use syn::Visibility;
use syn::ext::IdentExt;

/// The items of a library its `pub use` re-exports reach, wherever they are
/// defined, told by name as the library's types are: an item a reachable
/// `pub use` names, and a `use` binding that name -- a re-export re-exported
/// --; every item of a module a reachable glob opens, or that a reachable
/// `pub use` names, and of the `pub mod`s in it. What is reached is followed
/// until nothing more is, and comes out public.
#[derive(Default)]
pub struct ReExportReach {
    names: BTreeSet<String>,
    modules: BTreeSet<String>,
}

impl ReExportReach {
    pub fn of<'a>(&mut self, items: Vec<PlacedItem<'a>>) -> Vec<PlacedItem<'a>> {
        while self.follow(&items) {}
        items
            .into_iter()
            .map(|placed| PlacedItem {
                public: self.reaches(&placed),
                ..placed
            })
            .collect()
    }

    fn follow(&mut self, items: &[PlacedItem<'_>]) -> bool {
        let known = self.names.len() + self.modules.len();
        let reached = items
            .iter()
            .filter(|placed| self.reaches(placed))
            .collect::<Vec<&PlacedItem<'_>>>();
        for placed in reached {
            self.open(placed);
        }
        self.names.len() + self.modules.len() > known
    }

    fn named(&self, item: &Item) -> bool {
        match item {
            Item::Use(item) => UseLeaves::new(item)
                .of(&item.tree)
                .into_iter()
                .filter_map(|leaf| leaf.bound)
                .any(|bound| self.names.contains(&bound)),
            _ => ItemName
                .of(item)
                .is_some_and(|ident| self.names.contains(&ident.unraw().to_string())),
        }
    }

    fn open(&mut self, placed: &PlacedItem<'_>) {
        match placed.item {
            Item::Use(item) if matches!(item.vis, Visibility::Public(_)) => {
                self.names.extend(ReExports.names(item));
                self.modules.extend(ReExports.modules(item));
            }
            Item::Mod(item) if matches!(item.vis, Visibility::Public(_)) && !placed.public => {
                self.modules.insert(item.ident.unraw().to_string());
            }
            _ => {}
        }
    }

    fn reaches(&self, placed: &PlacedItem<'_>) -> bool {
        placed.public
            || placed
                .module
                .as_ref()
                .is_some_and(|module| self.modules.contains(module))
            || self.named(placed.item)
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::placed_item::PlacedItem;
use syn::Ident;
use syn::Item;
use syn::Path;
use syn::Type;
use syn::Visibility;

#[derive(Clone, Debug, PartialEq, Eq)]
struct LocalName {
    pub name: String,
    pub public: bool,
}

/// The types and traits a library defines, by name, and whether each is in
/// its public API: `pub`, in a module reachable through `pub mod`
/// declarations. A name defined twice is public where either is; a type or a
/// trait the library does not define is another crate's, and its users see it.
pub struct LocalTypes {
    names: Vec<LocalName>,
}

impl LocalTypes {
    pub fn new(items: &[PlacedItem<'_>]) -> Self {
        Self {
            names: items
                .iter()
                .filter_map(|placed| {
                    let (ident, visibility) = Self::defined(placed.item)?;
                    Some(LocalName {
                        name: ident.to_string(),
                        public: placed.public && matches!(visibility, Visibility::Public(_)),
                    })
                })
                .collect(),
        }
    }

    pub fn public_type(&self, ty: &Type) -> bool {
        Self::name_of(ty).is_some_and(|name| {
            self.names
                .iter()
                .any(|local| local.name == name && local.public)
        })
    }

    pub fn reachable(&self, ty: &Type) -> bool {
        Self::name_of(ty).is_none_or(|name| self.reachable_name(&name))
    }

    pub fn reachable_trait(&self, path: &Path) -> bool {
        path.segments
            .last()
            .is_none_or(|last| self.reachable_name(&last.ident.to_string()))
    }

    fn defined(item: &Item) -> Option<(&Ident, &Visibility)> {
        match item {
            Item::Enum(item) => Some((&item.ident, &item.vis)),
            Item::Struct(item) => Some((&item.ident, &item.vis)),
            Item::Trait(item) => Some((&item.ident, &item.vis)),
            Item::Type(item) => Some((&item.ident, &item.vis)),
            Item::Union(item) => Some((&item.ident, &item.vis)),
            _ => None,
        }
    }

    fn name_of(ty: &Type) -> Option<String> {
        match ty {
            Type::Path(path) => path.path.segments.last().map(|last| last.ident.to_string()),
            Type::Reference(reference) => Self::name_of(&reference.elem),
            _ => None,
        }
    }

    fn reachable_name(&self, name: &str) -> bool {
        let mut local = self
            .names
            .iter()
            .filter(|local| local.name == name)
            .peekable();
        local.peek().is_none() || local.any(|local| local.public)
    }
}

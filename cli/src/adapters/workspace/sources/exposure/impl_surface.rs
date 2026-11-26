// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::fragment::Fragment;
use crate::adapters::workspace::sources::exposure::local_types::LocalTypes;
use syn::ImplItem;
use syn::ItemImpl;
use syn::Path;
use syn::Visibility;

/// What a crate's users see of an `impl` block, wherever in the library it
/// is written: a trait's implementation where neither the trait nor the type
/// is the library's own and private -- its generics, the trait's path, the
/// type, its associated types --; a public type's own methods and constants
/// where they are `pub` -- their signatures and types.
pub struct ImplSurface<'a> {
    types: &'a LocalTypes,
}

impl<'a> ImplSurface<'a> {
    pub fn new(types: &'a LocalTypes) -> Self {
        Self { types }
    }

    pub fn implemented<'b>(&self, item: &'b ItemImpl, path: &'b Path) -> Vec<Fragment<'b>> {
        if !self.types.reachable_trait(path) || !self.types.reachable(&item.self_ty) {
            return Vec::new();
        }
        let types = item.items.iter().filter_map(|inner| match inner {
            ImplItem::Type(inner) => Some(Fragment::Type(&inner.ty)),
            _ => None,
        });
        [
            Fragment::Generics(&item.generics),
            Fragment::Path(path),
            Fragment::Type(&item.self_ty),
        ]
        .into_iter()
        .chain(types)
        .collect()
    }

    pub fn inherent<'b>(&self, item: &'b ItemImpl) -> Vec<Fragment<'b>> {
        if !self.types.public_type(&item.self_ty) {
            return Vec::new();
        }
        let items = item.items.iter().filter_map(|inner| match inner {
            ImplItem::Const(inner) if matches!(inner.vis, Visibility::Public(_)) => {
                Some(Fragment::Type(&inner.ty))
            }
            ImplItem::Fn(inner) if matches!(inner.vis, Visibility::Public(_)) => {
                Some(Fragment::Signature(&inner.sig))
            }
            _ => None,
        });
        [
            Fragment::Generics(&item.generics),
            Fragment::Type(&item.self_ty),
        ]
        .into_iter()
        .chain(items)
        .collect()
    }

    pub fn of<'b>(&self, item: &'b ItemImpl) -> Vec<Fragment<'b>> {
        match &item.trait_ {
            Some((_, path, _)) => self.implemented(item, path),
            None => self.inherent(item),
        }
    }
}

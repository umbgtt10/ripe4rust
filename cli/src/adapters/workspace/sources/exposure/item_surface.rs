// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::fragment::Fragment;
use syn::Attribute;
use syn::Fields;
use syn::Item;
use syn::ItemEnum;
use syn::ItemStruct;
use syn::ItemTrait;
use syn::ItemUnion;
use syn::TraitItem;
use syn::Visibility;

const DERIVES: [&str; 2] = ["cfg_attr", "derive"];

/// What a crate's users see of one of its items, where the item is `pub`:
/// a function's signature; a struct's or a union's generics, public fields
/// and derives; an enum's generics, every variant's fields and derives; a
/// trait's generics, bounds and its items' signatures, types and bounds; a
/// type alias, a constant's or a static's type; a `pub use` or a `pub extern
/// crate`. Bodies, private fields and private items are not seen.
pub struct ItemSurface;

impl ItemSurface {
    pub fn derives<'a>(&self, attributes: &'a [Attribute]) -> Vec<Fragment<'a>> {
        attributes
            .iter()
            .filter(|attribute| DERIVES.iter().any(|name| attribute.path().is_ident(name)))
            .map(Fragment::Attribute)
            .collect()
    }

    pub fn enumeration<'a>(&self, item: &'a ItemEnum) -> Vec<Fragment<'a>> {
        let fields = item
            .variants
            .iter()
            .flat_map(|variant| variant.fields.iter())
            .map(|field| Fragment::Type(&field.ty));
        [Fragment::Generics(&item.generics)]
            .into_iter()
            .chain(self.derives(&item.attrs))
            .chain(fields)
            .collect()
    }

    pub fn of<'a>(&self, item: &'a Item) -> Vec<Fragment<'a>> {
        match item {
            Item::Const(item) => self.public(&item.vis, vec![Fragment::Type(&item.ty)]),
            Item::Enum(item) => self.public(&item.vis, self.enumeration(item)),
            Item::ExternCrate(item) => self.public(&item.vis, vec![Fragment::ExternCrate(item)]),
            Item::Fn(item) => self.public(&item.vis, vec![Fragment::Signature(&item.sig)]),
            Item::Static(item) => self.public(&item.vis, vec![Fragment::Type(&item.ty)]),
            Item::Struct(item) => self.public(&item.vis, self.structure(item)),
            Item::Trait(item) => self.public(&item.vis, self.trait_items(item)),
            Item::Type(item) => self.public(
                &item.vis,
                vec![Fragment::Generics(&item.generics), Fragment::Type(&item.ty)],
            ),
            Item::Union(item) => self.public(&item.vis, self.union(item)),
            Item::Use(item) => self.public(&item.vis, vec![Fragment::Use(item)]),
            _ => Vec::new(),
        }
    }

    pub fn public<'a>(
        &self,
        visibility: &Visibility,
        fragments: Vec<Fragment<'a>>,
    ) -> Vec<Fragment<'a>> {
        if matches!(visibility, Visibility::Public(_)) {
            fragments
        } else {
            Vec::new()
        }
    }

    pub fn structure<'a>(&self, item: &'a ItemStruct) -> Vec<Fragment<'a>> {
        [Fragment::Generics(&item.generics)]
            .into_iter()
            .chain(self.derives(&item.attrs))
            .chain(Self::fields(&item.fields))
            .collect()
    }

    pub fn trait_items<'a>(&self, item: &'a ItemTrait) -> Vec<Fragment<'a>> {
        let items = item.items.iter().flat_map(|inner| match inner {
            TraitItem::Const(inner) => vec![Fragment::Type(&inner.ty)],
            TraitItem::Fn(inner) => vec![Fragment::Signature(&inner.sig)],
            TraitItem::Type(inner) => [Fragment::Generics(&inner.generics)]
                .into_iter()
                .chain(inner.bounds.iter().map(Fragment::Bound))
                .chain(inner.default.iter().map(|(_, ty)| Fragment::Type(ty)))
                .collect(),
            _ => Vec::new(),
        });
        [Fragment::Generics(&item.generics)]
            .into_iter()
            .chain(item.supertraits.iter().map(Fragment::Bound))
            .chain(items)
            .collect()
    }

    pub fn union<'a>(&self, item: &'a ItemUnion) -> Vec<Fragment<'a>> {
        [Fragment::Generics(&item.generics)]
            .into_iter()
            .chain(self.derives(&item.attrs))
            .chain(
                item.fields
                    .named
                    .iter()
                    .filter(|field| matches!(field.vis, Visibility::Public(_)))
                    .map(|field| Fragment::Type(&field.ty)),
            )
            .collect()
    }

    fn fields(fields: &Fields) -> Vec<Fragment<'_>> {
        fields
            .iter()
            .filter(|field| matches!(field.vis, Visibility::Public(_)))
            .map(|field| Fragment::Type(&field.ty))
            .collect()
    }
}

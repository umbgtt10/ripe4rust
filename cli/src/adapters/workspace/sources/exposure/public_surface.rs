// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::impl_surface::ImplSurface;
use crate::adapters::workspace::sources::exposure::item_surface::ItemSurface;
use crate::adapters::workspace::sources::exposure::local_types::LocalTypes;
use crate::adapters::workspace::sources::exposure::module_items::ModuleItems;
use crate::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use crate::adapters::workspace::sources::exposure::surface_names::SurfaceNames;
use syn::Item;

/// The dependencies in a library's public API, by package: those its `pub`
/// items in modules reachable from its root through `pub mod` declarations
/// name where its users see them, and those its trait implementations and
/// public types' methods name, wherever in the library they are written.
pub struct PublicSurface;

impl PublicSurface {
    pub fn packages(&self, files: &[ScopedFile<'_>]) -> Vec<String> {
        let items = ModuleItems.of(files);
        let types = LocalTypes::new(&items);
        let implemented = ImplSurface::new(&types);
        let mut packages = items
            .iter()
            .flat_map(|placed| {
                let mut fragments = match placed.item {
                    Item::Impl(item) => implemented.of(item),
                    _ => Vec::new(),
                };
                if placed.public {
                    fragments.extend(ItemSurface.of(placed.item));
                }
                SurfaceNames::new(placed.scope).of(&fragments)
            })
            .collect::<Vec<String>>();
        packages.sort();
        packages.dedup();
        packages
    }
}

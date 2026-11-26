// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::impl_surface::ImplSurface;
use cargo_ripe4rust::adapters::workspace::sources::exposure::local_types::LocalTypes;
use cargo_ripe4rust::adapters::workspace::sources::exposure::module_items::ModuleItems;
use cargo_ripe4rust::adapters::workspace::sources::exposure::surface_names::SurfaceNames;
use syn::Item;
use syn::ItemImpl;

const TYPES: &str = "pub struct Shown;\nstruct Hidden;\npub trait Open {}\ntrait Closed {}\n";

fn implemented(
    text: &str,
    run: impl Fn(&ImplSurface<'_>, &ItemImpl, &SurfaceNames<'_>) -> Vec<String>,
) -> Vec<Vec<String>> {
    let file = parsed(&format!("{TYPES}{text}"));
    scoped("", |scope| {
        let items = ModuleItems::items(&file.items, true, None, scope);
        let types = LocalTypes::new(&items);
        let surface = ImplSurface::new(&types);
        let names = SurfaceNames::new(scope);
        file.items
            .iter()
            .filter_map(|item| match item {
                Item::Impl(item) => Some(run(&surface, item, &names)),
                _ => None,
            })
            .collect()
    })
}

fn names(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| String::from(*name)).collect()
}

#[test]
fn implemented_of_a_trait_on_a_type_only_where_neither_is_ours_and_private() {
    // Arrange & Act
    let found = implemented(
        "impl serde::Serialize for Shown {}\nimpl serde::Serialize for Hidden {}\nimpl Open for itoa::Buffer {}\nimpl Closed for url::Url {}\nimpl<T: log::Log> Open for T {\n    type A = smol_str::SmolStr;\n}\n",
        |surface, item, names| {
            let Some((_, path, _)) = &item.trait_ else {
                return Vec::new();
            };
            names.of(&surface.implemented(item, path))
        },
    );

    // Assert
    assert_eq!(
        found,
        vec![
            names(&["serde"]),
            Vec::new(),
            names(&["itoa"]),
            Vec::new(),
            names(&["log", "smol-str"]),
        ]
    );
}

#[test]
fn inherent_of_a_public_type_is_its_public_methods_and_constants() {
    // Arrange & Act
    let found = implemented(
        "impl Shown {\n    pub const A: itoa::Buffer = todo!();\n    pub fn b(&self) -> url::Url {\n        log::info!(\"x\");\n        todo!()\n    }\n    fn c(&self) -> serde::Thing {\n        todo!()\n    }\n}\nimpl Hidden {\n    pub fn d(&self) -> smol_str::SmolStr {\n        todo!()\n    }\n}\n",
        |surface, item, names| names.of(&surface.inherent(item)),
    );

    // Assert
    assert_eq!(found, vec![names(&["itoa", "url"]), Vec::new()]);
}

#[test]
fn of_an_impl_is_its_trait_s_or_its_type_s_own() {
    // Arrange & Act
    let found = implemented(
        "impl &Shown {\n    pub fn a(&self) -> itoa::Buffer {\n        todo!()\n    }\n}\nimpl serde::Serialize for &Shown {}\n",
        |surface, item, names| names.of(&surface.of(item)),
    );

    // Assert
    assert_eq!(found, vec![names(&["itoa"]), names(&["serde"])]);
}

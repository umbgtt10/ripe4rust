// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::fragment::Fragment;
use cargo_ripe4rust::adapters::workspace::sources::exposure::item_surface::ItemSurface;
use cargo_ripe4rust::adapters::workspace::sources::exposure::surface_names::SurfaceNames;
use syn::Item;

fn first(text: &str) -> Item {
    parsed(text).items.remove(0)
}

fn named(fragments: &[Fragment<'_>]) -> Vec<String> {
    scoped("", |scope| SurfaceNames::new(scope).of(fragments))
}

#[test]
fn derives_of_attributes_are_the_derives_and_the_conditional_ones() {
    // Arrange
    let item = first(
        "#[derive(serde::Serialize)]\n#[cfg_attr(test, derive(itoa::Thing))]\n#[doc = \"url::Url\"]\n#[log::trace]\nstruct A;\n",
    );
    let Item::Struct(structure) = &item else {
        panic!("a struct");
    };

    // Act
    let fragments = ItemSurface.derives(&structure.attrs);

    // Assert
    assert_eq!(named(&fragments), ["itoa", "serde"].map(String::from));
}

#[test]
fn enumeration_of_an_enum_is_its_generics_derives_and_every_variant_s_fields() {
    // Arrange
    let item = first(
        "#[derive(serde::Serialize)]\nenum A<T: log::Log> {\n    B(itoa::Buffer),\n    C { d: url::Url },\n    E,\n}\n",
    );
    let Item::Enum(enumeration) = &item else {
        panic!("an enum");
    };

    // Act
    let fragments = ItemSurface.enumeration(enumeration);

    // Assert
    assert_eq!(
        named(&fragments),
        ["itoa", "log", "serde", "url"].map(String::from)
    );
}

#[test]
fn of_a_private_item_is_nothing() {
    // Arrange
    let items = parsed(
        "const A: itoa::Buffer = todo!();\nfn b(c: url::Url) {}\npub(crate) struct D(pub log::Level);\nuse serde::Serialize;\n",
    )
    .items;

    // Act
    let fragments = items.iter().flat_map(|item| ItemSurface.of(item)).count();

    // Assert
    assert_eq!(fragments, 0);
}

#[test]
fn of_every_kind_of_public_item_is_what_its_users_see() {
    // Arrange
    let items = parsed(
        "pub const A: itoa::Buffer = todo!();\npub enum B {\n    C(log::Level),\n}\npub extern crate smol_str;\npub fn d(e: url::Url) {}\npub static F: itertools::Thing = todo!();\npub struct G(pub tokio::Runtime);\npub trait H: serde::Serialize {}\npub type I = itoa::Thing;\npub union J {\n    pub k: url::Thing,\n}\npub use log::Log;\npub mod hidden {}\nimpl G {}\n",
    )
    .items;

    // Act
    let fragments = items
        .iter()
        .flat_map(|item| ItemSurface.of(item))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        named(&fragments),
        [
            "itertools",
            "itoa",
            "log",
            "serde",
            "smol-str",
            "tokio",
            "url"
        ]
        .map(String::from)
    );
}

#[test]
fn public_of_a_visibility_short_of_pub_drops_the_fragments() {
    // Arrange
    let items = parsed("pub(crate) fn a(b: itoa::Buffer) {}\npub fn c(d: url::Url) {}\n").items;
    let (Item::Fn(hidden), Item::Fn(shown)) = (&items[0], &items[1]) else {
        panic!("two functions");
    };

    // Act
    let kept = [hidden, shown].map(|function| {
        ItemSurface
            .public(&function.vis, vec![Fragment::Signature(&function.sig)])
            .len()
    });

    // Assert
    assert_eq!(kept, [0, 1]);
}

#[test]
fn structure_of_a_struct_is_its_generics_derives_and_public_fields() {
    // Arrange
    let item = first(
        "#[derive(Debug, serde::Serialize)]\nstruct A<T: log::Log> {\n    pub a: itoa::Buffer,\n    b: url::Url,\n    c: T,\n}\n",
    );
    let Item::Struct(structure) = &item else {
        panic!("a struct");
    };

    // Act
    let fragments = ItemSurface.structure(structure);

    // Assert
    assert_eq!(
        named(&fragments),
        ["itoa", "log", "serde"].map(String::from)
    );
}

#[test]
fn trait_items_of_a_trait_are_its_bounds_and_its_items_signatures() {
    // Arrange
    let item = first(
        "trait A<T: tokio::Runtime>: serde::Serialize {\n    type B<C: url::Thing>: itertools::Itertools = smol_str::SmolStr;\n    const D: itoa::Buffer;\n    fn e(&self) -> log::Level {\n        url::Url::parse(\"x\");\n        todo!()\n    }\n    some::mac!();\n}\n",
    );
    let Item::Trait(traited) = &item else {
        panic!("a trait");
    };

    // Act
    let fragments = ItemSurface.trait_items(traited);

    // Assert
    assert_eq!(
        named(&fragments),
        [
            "itertools",
            "itoa",
            "log",
            "serde",
            "smol-str",
            "tokio",
            "url"
        ]
        .map(String::from)
    );
}

#[test]
fn union_of_a_union_is_its_generics_derives_and_public_fields() {
    // Arrange
    let item = first(
        "#[derive(serde::Serialize)]\nunion A<T: Copy> {\n    pub a: itoa::Buffer,\n    b: url::Url,\n    c: T,\n}\n",
    );
    let Item::Union(union) = &item else {
        panic!("a union");
    };

    // Act
    let fragments = ItemSurface.union(union);

    // Assert
    assert_eq!(named(&fragments), ["itoa", "serde"].map(String::from));
}

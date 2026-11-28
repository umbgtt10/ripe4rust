// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::exposure::fragment::Fragment;
use cargo_ripe4rust::adapters::workspace::sources::exposure::surface_names::SurfaceNames;
use syn::Item;

#[test]
fn of_fragments_is_every_crate_they_name_once() {
    // Arrange
    let file = parsed(
        "#[derive(serde::Serialize)]\npub struct A<T: itertools::Itertools>(pub itoa::Buffer, T);\nimpl log::Log for A<u8> {}\npub fn b(c: itoa::Buffer) {}\npub use url::Url;\npub extern crate smol_str;\ntrait D: tokio::Runtime {}\n",
    );
    let (
        Item::Struct(structure),
        Item::Impl(implemented),
        Item::Fn(function),
        Item::Use(used),
        Item::ExternCrate(external),
        Item::Trait(traited),
    ) = (
        &file.items[0],
        &file.items[1],
        &file.items[2],
        &file.items[3],
        &file.items[4],
        &file.items[5],
    )
    else {
        panic!("the items");
    };
    let Some((path, _)) = &implemented.trait_ else {
        panic!("a trait");
    };
    let fragments = [
        Fragment::Attribute(&structure.attrs[0]),
        Fragment::Generics(&structure.generics),
        Fragment::Type(&implemented.self_ty),
        Fragment::Path(path),
        Fragment::Signature(&function.sig),
        Fragment::Use(used),
        Fragment::ExternCrate(external),
        Fragment::Bound(&traited.supertraits[0]),
    ];

    // Act
    let packages = scoped("", |scope| SurfaceNames::new(scope).of(&fragments));

    // Assert
    assert_eq!(
        packages,
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

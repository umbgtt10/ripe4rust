// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::exposed;
use crate::support::sources::exposed_in;

fn names(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| String::from(*name)).collect()
}

#[test]
fn packages_of_a_crate_s_trait_implemented_on_a_public_type_are_that_crate() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub struct A;\nimpl serde::Serialize for A {}\nstruct B;\nimpl url::Visit for B {}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["serde"]));
}

#[test]
fn packages_of_a_public_enum_are_every_variant_s_fields() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub enum A {\n    B(itoa::Buffer),\n    C { c: url::Url },\n}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa", "url"]));
}

#[test]
fn packages_of_a_public_function_are_the_crates_its_signature_names() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub fn digits(value: itoa::Buffer) -> url::Url {\n    let _ = log::Level::Info;\n    todo!()\n}\nfn hidden(value: smol_str::SmolStr) {}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa", "url"]));
}

#[test]
fn packages_of_a_public_struct_are_its_public_fields_and_its_derives() {
    // Arrange & Act
    let packages = exposed(&[(
        "use serde::Serialize;\n#[derive(Debug, Serialize)]\npub struct A {\n    pub a: itoa::Buffer,\n    b: url::Url,\n}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa", "serde"]));
}

#[test]
fn packages_of_a_public_trait_are_its_bounds_and_its_items_signatures() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub trait A: serde::Serialize {\n    type B: itertools::Itertools;\n    const C: itoa::Buffer;\n    fn d(&self) -> url::Url {\n        log::info!(\"x\");\n        todo!()\n    }\n}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itertools", "itoa", "serde", "url"]));
}

#[test]
fn packages_of_a_public_trait_implemented_on_a_crate_s_type_are_that_crate() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub trait A {}\nimpl A for itoa::Buffer {}\ntrait B {}\nimpl B for url::Url {}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa"]));
}

#[test]
fn packages_of_a_public_type_s_methods_are_their_public_signatures() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub struct A;\nimpl A {\n    pub fn b(&self) -> itoa::Buffer {\n        todo!()\n    }\n    fn c(&self) -> url::Url {\n        todo!()\n    }\n}\nstruct D;\nimpl D {\n    pub fn e(&self) -> log::Level {\n        todo!()\n    }\n}\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa"]));
}

#[test]
fn packages_of_a_type_alias_and_of_re_exports_are_what_they_name() {
    // Arrange & Act
    let packages = exposed(&[(
        "pub type A = itoa::Buffer;\npub use url::Url;\npub use log;\npub const B: smol_str::SmolStr = todo!();\nuse tokio::Runtime;\n",
        true,
    )]);

    // Assert
    assert_eq!(packages, names(&["itoa", "log", "smol-str", "url"]));
}

#[test]
fn packages_of_an_impl_in_a_private_file_on_a_public_type_are_its_trait_s() {
    // Arrange & Act
    let packages = exposed(&[
        ("pub struct A;\nmod hidden;\n", true),
        (
            "impl serde::Serialize for crate::A {}\nfn b(c: url::Url) {}\n",
            false,
        ),
    ]);

    // Assert
    assert_eq!(packages, names(&["serde"]));
}

#[test]
fn packages_of_items_a_glob_re_exports_from_a_private_module_are_its_public_items() {
    // Arrange & Act
    let packages = exposed_in(&[
        ("mod inner;\npub use inner::*;\n", true, None),
        (
            "pub fn a(b: itoa::Buffer) {}\nfn c(d: url::Url) {}\n",
            false,
            Some("inner"),
        ),
    ]);

    // Assert
    assert_eq!(packages, names(&["itoa"]));
}

#[test]
fn packages_of_items_re_exported_from_a_private_module_are_theirs_and_their_methods() {
    // Arrange & Act
    let packages = exposed_in(&[
        ("mod inner;\npub use inner::Thing;\n", true, None),
        (
            "pub struct Thing {\n    pub a: itoa::Buffer,\n}\nimpl Thing {\n    pub fn b(&self) -> url::Url {\n        todo!()\n    }\n}\nimpl serde::Serialize for Thing {}\npub fn c(d: log::Level) {}\n",
            false,
            Some("inner"),
        ),
    ]);

    // Assert
    assert_eq!(packages, names(&["itoa", "serde", "url"]));
}

#[test]
fn packages_of_items_reachable_only_through_public_modules_are_theirs() {
    // Arrange & Act
    let packages = exposed(&[
        (
            "pub mod outer {\n    pub fn a(b: itoa::Buffer) {}\n}\nmod inner {\n    pub fn c(d: url::Url) {}\n}\n",
            true,
        ),
        ("pub fn e(f: log::Level) {}\n", false),
    ]);

    // Assert
    assert_eq!(packages, names(&["itoa"]));
}

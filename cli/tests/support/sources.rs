// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::sources::exposure::public_surface::PublicSurface;
use cargo_ripe4rust::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::declaration_collector::DeclarationCollector;
use cargo_ripe4rust::adapters::workspace::sources::naming::declarations::Declarations;
use cargo_ripe4rust::adapters::workspace::sources::naming::import_scope::ImportScope;
use cargo_ripe4rust::adapters::workspace::sources::naming::name_finder::NameFinder;
use cargo_ripe4rust::adapters::workspace::sources::naming::named::Named;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;
use cargo_ripe4rust::adapters::workspace::sources::walking::parsed_file::ParsedFile;
use cargo_ripe4rust::system::model::declared_dependency::DeclaredDependency;
use cargo_ripe4rust::system::model::member_sources::MemberSources;
use std::path::PathBuf;
use std::slice::from_ref;
use syn::File;
use syn::parse_file;
use syn::visit::Visit;

pub const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

/// `app` using itoa, itertools, log, serde, smol-str, tokio and url, every one
/// from crates.io.
pub fn app() -> MemberSources {
    member(
        [
            "itertools",
            "itoa",
            "log",
            "serde",
            "smol-str",
            "tokio",
            "url",
        ]
        .iter()
        .map(|name| dependency(name, None))
        .collect(),
    )
}

/// The declarations of one file.
pub fn declared(text: &str) -> Declarations {
    DeclarationCollector::of(&parsed(text))
}

/// A crates.io dependency, renamed where `rename` says.
pub fn dependency(name: &str, rename: Option<&str>) -> DeclaredDependency {
    DeclaredDependency {
        name: String::from(name),
        rename: rename.map(String::from),
        source: Some(String::from(CRATES_IO)),
    }
}

/// The crates in the public API of a library whose files are these texts,
/// each with whether it is reachable through `pub mod` declarations; the
/// first is its `lib.rs`.
pub fn exposed(texts: &[(&str, bool)]) -> Vec<String> {
    exposed_in(
        &texts
            .iter()
            .map(|(text, public)| (*text, *public, None))
            .collect::<Vec<_>>(),
    )
}

/// The crates in the public API of a library whose files are these texts,
/// each with whether it is reachable through `pub mod` declarations and the
/// module it holds; the first is its `lib.rs`.
pub fn exposed_in(texts: &[(&str, bool, Option<&str>)]) -> Vec<String> {
    let member = app();
    let codes = CodeNames::new(&member);
    let files = texts
        .iter()
        .enumerate()
        .map(|(at, (text, public, module))| ParsedFile {
            path: PathBuf::from(format!("/work/ws/app/src/{at}.rs")),
            name: format!("app/src/{at}.rs"),
            syntax: parsed(text),
            public: *public,
            module: module.map(String::from),
        })
        .collect::<Vec<ParsedFile>>();
    let declarations = files
        .iter()
        .map(|file| DeclarationCollector::of(&file.syntax))
        .collect::<Vec<Declarations>>();
    let macros = TargetMacros::new(&declarations, &codes);
    let scopes = declarations
        .iter()
        .map(|declared| ImportScope::new(declared, &codes, &macros))
        .collect::<Vec<ImportScope<'_>>>();
    let library = files
        .iter()
        .zip(&scopes)
        .map(|(file, scope)| ScopedFile { file, scope })
        .collect::<Vec<ScopedFile<'_>>>();
    PublicSurface.packages(&library)
}

/// Lines naming dependencies, as `package:line`, in order.
pub fn lines(found: &[Named]) -> Vec<String> {
    let mut lines = found
        .iter()
        .map(|named| format!("{}:{}", named.package, named.line))
        .collect::<Vec<String>>();
    lines.sort();
    lines
}

/// A member named `app` with these dependencies and no targets.
pub fn member(dependencies: Vec<DeclaredDependency>) -> MemberSources {
    MemberSources {
        name: String::from("app"),
        targets: Vec::new(),
        dependencies,
    }
}

/// The lines of one file naming each dependency of `member`, as `package:line`.
pub fn named(text: &str, member: &MemberSources) -> Vec<String> {
    let file = parsed(text);
    let declarations = DeclarationCollector::of(&file);
    let codes = CodeNames::new(member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);
    let mut finder = NameFinder::new(&scope);
    finder.visit_file(&file);
    finder
        .found()
        .iter()
        .map(|named| format!("{}:{}", named.package, named.line))
        .collect()
}

pub fn parsed(text: &str) -> File {
    parse_file(text).expect("a file")
}

/// What `run` makes of the scope of a file of `app` holding `imports`.
pub fn scoped<T>(imports: &str, run: impl FnOnce(&ImportScope<'_>) -> T) -> T {
    let declarations = declared(imports);
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);
    run(&scope)
}

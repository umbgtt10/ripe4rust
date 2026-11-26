// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use crate::support::sources::app;
use crate::support::sources::declared;
use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use cargo_ripe4rust::adapters::workspace::sources::member_scanner::MemberScanner;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::adapters::workspace::sources::naming::import_scope::ImportScope;
use cargo_ripe4rust::adapters::workspace::sources::naming::target_macros::TargetMacros;
use cargo_ripe4rust::adapters::workspace::sources::walking::parsed_file::ParsedFile;
use cargo_ripe4rust::system::model::member_sources::MemberSources;
use cargo_ripe4rust::system::model::source_target::SourceTarget;
use ripe4rust_core::values::location::Location;
use std::path::Path;
use std::path::PathBuf;
use std::slice::from_ref;

const LIBRARY: &str = "pub fn digits(buffer: itoa::Buffer) -> String {\n    url::Url::parse(\"x\");\n    todo!()\n}\nmod hidden;\n";

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

fn target(kind: &str, root: &str) -> SourceTarget {
    SourceTarget {
        kinds: vec![String::from(kind)],
        root: PathBuf::from(root),
    }
}

fn with_targets(targets: Vec<SourceTarget>) -> MemberSources {
    MemberSources { targets, ..app() }
}

#[test]
fn named_of_a_file_is_each_line_naming_a_dependency_in_that_file() {
    // Arrange
    let files = MemoryFiles::new();
    let declarations = declared("use itoa::Buffer;\n");
    let member = app();
    let codes = CodeNames::new(&member);
    let macros = TargetMacros::new(from_ref(&declarations), &codes);
    let scope = ImportScope::new(&declarations, &codes, &macros);
    let file = ParsedFile {
        path: PathBuf::from("/work/ws/app/src/lib.rs"),
        name: String::from("app/src/lib.rs"),
        syntax: parsed("use itoa::Buffer;\n"),
        public: true,
        module: None,
    };

    // Act
    let named = MemberScanner::new(&files, Path::new("/work/ws")).named(&ScopedFile {
        file: &file,
        scope: &scope,
    });

    // Assert
    assert_eq!(named, vec![(String::from("itoa"), at("app/src/lib.rs", 1))]);
}

#[test]
fn scan_of_a_member_including_generated_code_counts_what_that_code_names() {
    // Arrange
    let files = MemoryFiles::new()
        .with(
            "/work/ws/app/src/lib.rs",
            "pub mod wire {\n    include!(\"generated/wire.rs\");\n}\n",
        )
        .with(
            "/work/ws/app/src/generated/wire.rs",
            "#[derive(Clone, PartialEq, ::serde::Serialize)]\npub struct Envelope {\n    pub level: ::log::Level,\n}\n",
        );
    let member = with_targets(vec![target("lib", "/work/ws/app/src/lib.rs")]);

    // Act
    let found = MemberScanner::new(&files, Path::new("/work/ws")).scan(&member);

    // Assert
    assert_eq!(
        (found.named, found.exposed),
        (
            vec![
                (String::from("log"), at("app/src/generated/wire.rs", 3)),
                (String::from("serde"), at("app/src/generated/wire.rs", 1)),
            ],
            vec![String::from("log"), String::from("serde")]
        )
    );
}

#[test]
fn scan_of_a_member_is_every_target_s_lines_and_its_library_s_api() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/app/src/lib.rs", LIBRARY)
        .with(
            "/work/ws/app/src/hidden.rs",
            "fn a() -> log::Level {\n    todo!()\n}\n",
        )
        .with("/work/ws/app/tests/it.rs", "\nuse tokio::Runtime;\n");
    let member = with_targets(vec![
        target("lib", "/work/ws/app/src/lib.rs"),
        target("test", "/work/ws/app/tests/it.rs"),
    ]);

    // Act
    let found = MemberScanner::new(&files, Path::new("/work/ws")).scan(&member);

    // Assert
    assert_eq!(
        (found.named, found.exposed, found.parsed, found.api_unread),
        (
            vec![
                (String::from("log"), at("app/src/hidden.rs", 1)),
                (String::from("itoa"), at("app/src/lib.rs", 1)),
                (String::from("url"), at("app/src/lib.rs", 2)),
                (String::from("tokio"), at("app/tests/it.rs", 2)),
            ],
            vec![String::from("itoa")],
            3,
            false
        )
    );
}

#[test]
fn target_of_a_binary_or_a_proc_macro_library_exports_no_api() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/app/src/lib.rs", LIBRARY);
    let member = app();
    let codes = CodeNames::new(&member);
    let scanner = MemberScanner::new(&files, Path::new("/work/ws"));

    // Act
    let exposed = ["bin", "proc-macro", "rlib"].map(|kind| {
        scanner
            .target(&target(kind, "/work/ws/app/src/lib.rs"), &codes)
            .exposed
    });

    // Assert
    assert_eq!(
        exposed,
        [Vec::new(), Vec::new(), vec![String::from("itoa")]]
    );
}

#[test]
fn target_of_a_library_with_a_file_it_could_not_read_has_its_api_unread() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/app/src/lib.rs", LIBRARY);
    let member = app();
    let codes = CodeNames::new(&member);
    let scanner = MemberScanner::new(&files, Path::new("/work/ws"));

    // Act
    let unread = ["lib", "bin"].map(|kind| {
        let found = scanner.target(&target(kind, "/work/ws/app/src/lib.rs"), &codes);
        (found.api_unread, found.unparsed.len())
    });

    // Assert
    assert_eq!(unread, [(true, 1), (false, 1)]);
}

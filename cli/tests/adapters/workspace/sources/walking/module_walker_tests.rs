// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::memory_files::MemoryFiles;
use cargo_ripe4rust::adapters::workspace::sources::walking::module_source::ModuleSource;
use cargo_ripe4rust::adapters::workspace::sources::walking::module_walker::ModuleWalker;
use cargo_ripe4rust::adapters::workspace::sources::walking::walk::Walk;
use ripe4rust_core::inputs::unparsed_file::UnparsedFile;
use ripe4rust_core::values::location::Location;
use std::path::Path;
use std::path::PathBuf;

fn at(file: &str, line: u32) -> Location {
    Location {
        file: String::from(file),
        line,
    }
}

fn parsed(walk: &Walk) -> Vec<(String, bool)> {
    walk.parsed
        .iter()
        .map(|file| (file.name.clone(), file.public))
        .collect()
}

fn unparsed(file: &str, at: Location, detail: &str) -> UnparsedFile {
    UnparsedFile {
        file: String::from(file),
        at,
        detail: String::from(detail),
    }
}

fn walked(files: &MemoryFiles, target: &str) -> Walk {
    ModuleWalker::new(files, Path::new("/work/ws")).walk(Path::new(target))
}

#[test]
fn children_of_a_file_are_its_modules_where_they_may_be() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/app/src/a.rs", "pub mod b;\n");
    let walker = ModuleWalker::new(&files, Path::new("/work/ws"));
    let source = ModuleSource {
        candidates: vec![PathBuf::from("/work/ws/app/src/a.rs")],
        owns_directory: false,
        public: true,
        declared_at: None,
        name: Some(String::from("a")),
    };
    let parsed = walker.read(&source).expect("a parsed file");

    // Act
    let children = walker.children(&parsed, &source);

    // Assert
    assert_eq!(
        children,
        vec![ModuleSource {
            candidates: vec![
                PathBuf::from("/work/ws/app/src/a/b.rs"),
                PathBuf::from("/work/ws/app/src/a/b/mod.rs"),
            ],
            owns_directory: false,
            public: true,
            declared_at: Some(at("app/src/a.rs", 1)),
            name: Some(String::from("b")),
        }]
    );
}

#[test]
fn included_of_a_file_are_the_files_its_includes_name_from_its_directory() {
    // Arrange
    let files = MemoryFiles::new().with(
        "/work/ws/app/src/net/wire.rs",
        "\nmod proto {\n    include!(\"../generated/proto.rs\");\n}\n",
    );
    let walker = ModuleWalker::new(&files, Path::new("/work/ws"));
    let parsed = walker
        .read(&ModuleSource {
            candidates: vec![PathBuf::from("/work/ws/app/src/net/wire.rs")],
            owns_directory: false,
            public: true,
            declared_at: None,
            name: Some(String::from("wire")),
        })
        .expect("a parsed file");

    // Act
    let included = walker.included(&parsed);

    // Assert
    assert_eq!(
        included,
        vec![ModuleSource {
            candidates: vec![PathBuf::from("/work/ws/app/src/generated/proto.rs")],
            owns_directory: false,
            public: false,
            declared_at: Some(at("app/src/net/wire.rs", 3)),
            name: Some(String::from("proto")),
        }]
    );
}

#[test]
fn read_of_a_module_is_the_first_of_its_files_there() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/app/src/b/mod.rs", "fn f() {}\n");
    let source = ModuleSource {
        candidates: vec![
            PathBuf::from("/work/ws/app/src/b.rs"),
            PathBuf::from("/work/ws/app/src/b/mod.rs"),
        ],
        owns_directory: false,
        public: false,
        declared_at: Some(at("app/src/lib.rs", 3)),
        name: Some(String::from("b")),
    };

    // Act
    let read = ModuleWalker::new(&files, Path::new("/work/ws"))
        .read(&source)
        .map(|parsed| {
            (
                parsed.name,
                parsed.module,
                parsed.public,
                parsed.syntax.items.len(),
            )
        });

    // Assert
    assert_eq!(
        read,
        Ok((
            String::from("app/src/b/mod.rs"),
            Some(String::from("b")),
            false,
            1
        ))
    );
}

#[test]
fn unfollowed_of_a_file_are_where_it_includes_code_the_parse_cannot_follow() {
    // Arrange
    let files = MemoryFiles::new().with(
        "/work/ws/app/src/wire.rs",
        "include!(\"followed.rs\");\n\ntonic::include_proto!(\"wire\");\n",
    );
    let walker = ModuleWalker::new(&files, Path::new("/work/ws"));
    let parsed = walker
        .read(&ModuleSource {
            candidates: vec![PathBuf::from("/work/ws/app/src/wire.rs")],
            owns_directory: false,
            public: true,
            declared_at: None,
            name: Some(String::from("wire")),
        })
        .expect("a parsed file");

    // Act
    let unfollowed = walker.unfollowed(&parsed);

    // Assert
    assert_eq!(unfollowed, vec![at("app/src/wire.rs", 3)]);
}

#[test]
fn walk_of_a_file_including_another_reads_it_in_the_module_it_is_included_in() {
    // Arrange
    let files = MemoryFiles::new()
        .with(
            "/work/ws/app-core/src/lib.rs",
            "pub mod transport;\n",
        )
        .with(
            "/work/ws/app-core/src/transport.rs",
            "pub mod wire {\n    include!(\"generated/wire.rs\");\n}\ninclude!(\"../shared/extra.rs\");\n",
        )
        .with("/work/ws/app-core/src/generated/wire.rs", "pub struct Envelope;\n")
        .with("/work/ws/app-core/shared/extra.rs", "");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        walk.parsed
            .iter()
            .map(|file| (file.name.as_str(), file.public, file.module.as_deref()))
            .collect::<Vec<_>>(),
        vec![
            ("app-core/shared/extra.rs", true, Some("transport")),
            ("app-core/src/generated/wire.rs", true, Some("wire")),
            ("app-core/src/lib.rs", true, None),
            ("app-core/src/transport.rs", true, Some("transport")),
        ]
    );
    assert_eq!(walk.unparsed, Vec::new());
}

#[test]
fn walk_of_a_file_including_code_it_cannot_follow_names_where() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/app/src/main.rs", "mod wire;\nfn main() {}\n")
        .with(
            "/work/ws/app/src/wire.rs",
            "\ninclude!(concat!(env!(\"OUT_DIR\"), \"/wire.rs\"));\n",
        );

    // Act
    let walk = walked(&files, "/work/ws/app/src/main.rs");

    // Assert
    assert_eq!(
        (walk.unfollowed, walk.unparsed),
        (vec![at("app/src/wire.rs", 2)], Vec::new())
    );
}

#[test]
fn walk_of_a_file_including_one_not_there_is_unparsed_at_the_include() {
    // Arrange
    let files = MemoryFiles::new().with(
        "/work/ws/app/src/main.rs",
        "fn main() {}\ninclude!(\"gone.rs\");\n",
    );

    // Act
    let walk = walked(&files, "/work/ws/app/src/main.rs");

    // Assert
    assert_eq!(
        walk.unparsed,
        vec![unparsed(
            "app/src/gone.rs",
            at("app/src/main.rs", 2),
            "it is not there"
        )]
    );
}

#[test]
fn walk_of_a_file_that_cannot_be_read_is_unparsed_at_its_first_line() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/app-core/src/lib.rs", "mod a;\n")
        .refusing("/work/ws/app-core/src/a.rs");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        walk.unparsed,
        vec![unparsed(
            "app-core/src/a.rs",
            at("app-core/src/a.rs", 1),
            "permission denied"
        )]
    );
}

#[test]
fn walk_of_a_file_that_does_not_parse_is_unparsed_at_the_line_it_stopped_at() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/app-core/src/lib.rs", "mod a;\n")
        .with("/work/ws/app-core/src/a.rs", "fn f() {\n    let x = ;\n}\n");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        (parsed(&walk), walk.unparsed),
        (
            vec![(String::from("app-core/src/lib.rs"), true)],
            vec![unparsed(
                "app-core/src/a.rs",
                at("app-core/src/a.rs", 2),
                "expected an expression"
            )]
        )
    );
}

#[test]
fn walk_of_a_library_names_each_file_s_module_as_its_mod_item_does() {
    // Arrange
    let files = MemoryFiles::new()
        .with(
            "/work/ws/app-core/src/lib.rs",
            "mod a;\nmod b {\n    pub mod c;\n}\n#[path = \"shared.rs\"]\nmod d;\n",
        )
        .with("/work/ws/app-core/src/a.rs", "")
        .with("/work/ws/app-core/src/b/c.rs", "")
        .with("/work/ws/app-core/src/shared.rs", "");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        walk.parsed
            .iter()
            .map(|file| (file.name.as_str(), file.module.as_deref()))
            .collect::<Vec<_>>(),
        vec![
            ("app-core/src/a.rs", Some("a")),
            ("app-core/src/b/c.rs", Some("c")),
            ("app-core/src/lib.rs", None),
            ("app-core/src/shared.rs", Some("d")),
        ]
    );
}

#[test]
fn walk_of_a_library_reads_every_module_it_declares_and_which_are_public() {
    // Arrange
    let files = MemoryFiles::new()
        .with("/work/ws/app-core/src/lib.rs", "pub mod a;\nmod b;\n")
        .with("/work/ws/app-core/src/a.rs", "pub mod c;\n")
        .with("/work/ws/app-core/src/a/c.rs", "pub fn f() {}\n")
        .with("/work/ws/app-core/src/b/mod.rs", "pub mod d;\n")
        .with("/work/ws/app-core/src/b/d.rs", "");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        (parsed(&walk), walk.unparsed),
        (
            vec![
                (String::from("app-core/src/a.rs"), true),
                (String::from("app-core/src/a/c.rs"), true),
                (String::from("app-core/src/b/d.rs"), false),
                (String::from("app-core/src/b/mod.rs"), false),
                (String::from("app-core/src/lib.rs"), true),
            ],
            Vec::new()
        )
    );
}

#[test]
fn walk_of_a_module_declared_twice_reads_its_file_once() {
    // Arrange
    let files = MemoryFiles::new()
        .with(
            "/work/ws/app/src/main.rs",
            "#[path = \"shared.rs\"]\nmod a;\n#[path = \"shared.rs\"]\nmod b;\n",
        )
        .with("/work/ws/app/src/shared.rs", "");

    // Act
    let walk = walked(&files, "/work/ws/app/src/main.rs");

    // Assert
    assert_eq!(
        parsed(&walk),
        vec![
            (String::from("app/src/main.rs"), true),
            (String::from("app/src/shared.rs"), false),
        ]
    );
}

#[test]
fn walk_of_a_module_not_there_is_unparsed_at_its_declaration() {
    // Arrange
    let files = MemoryFiles::new().with("/work/ws/app-core/src/lib.rs", "\nmod gone;\n");

    // Act
    let walk = walked(&files, "/work/ws/app-core/src/lib.rs");

    // Assert
    assert_eq!(
        walk.unparsed,
        vec![unparsed(
            "app-core/src/gone.rs",
            at("app-core/src/lib.rs", 2),
            "it is not there, nor `app-core/src/gone/mod.rs`"
        )]
    );
}

#[test]
fn walk_of_a_target_root_not_there_is_unparsed_at_its_first_line() {
    // Arrange
    let files = MemoryFiles::new();

    // Act
    let walk = walked(&files, "/work/ws/app/src/main.rs");

    // Assert
    assert_eq!(
        (walk.parsed.len(), walk.unparsed),
        (
            0,
            vec![unparsed(
                "app/src/main.rs",
                at("app/src/main.rs", 1),
                "it is not there"
            )]
        )
    );
}

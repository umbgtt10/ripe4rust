// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::walking::include_finder::IncludeFinder;
use cargo_ripe4rust::adapters::workspace::sources::walking::included_file::IncludedFile;
use syn::visit::Visit;

fn included(path: &str, line: u32, public: bool, module: Option<&str>) -> IncludedFile {
    IncludedFile {
        path: String::from(path),
        line,
        public,
        module: module.map(String::from),
    }
}

#[test]
fn found_of_a_file_is_every_include_of_a_literal_path_in_the_module_it_is_in() {
    // Arrange
    let file = parsed(
        "include!(\"top.rs\");\npub mod wire {\n    include!(\"generated/faction.rs\");\n}\nmod hidden {\n    std::include!(\"hidden.rs\");\n}\nconst DATA: &str = include_str!(\"data.txt\");\n",
    );
    let mut finder = IncludeFinder::new(true, Some(String::from("transport")));

    // Act
    finder.visit_file(&file);

    // Assert
    assert_eq!(
        finder.found(),
        vec![
            included("top.rs", 1, true, Some("transport")),
            included("generated/faction.rs", 3, true, Some("wire")),
            included("hidden.rs", 6, false, Some("hidden")),
        ]
    );
}

#[test]
fn found_of_a_file_outside_the_public_api_is_never_public() {
    // Arrange
    let file = parsed("pub mod wire {\n    include!(\"wire.rs\");\n}\n");
    let mut finder = IncludeFinder::new(false, None);

    // Act
    finder.visit_file(&file);

    // Assert
    assert_eq!(
        finder.found(),
        vec![included("wire.rs", 2, false, Some("wire"))]
    );
}

#[test]
fn unfollowed_of_a_file_are_the_includes_it_cannot_follow() {
    // Arrange
    let file = parsed(
        "include!(concat!(env!(\"OUT_DIR\"), \"/built.rs\"));\ntonic::include_proto!(\"faction\");\nfn value() -> u32 {\n    include!(\"value.rs\")\n}\ninclude!(\"followed.rs\");\nconst DATA: &[u8] = include_bytes!(\"data.bin\");\n",
    );
    let mut finder = IncludeFinder::new(true, None);

    // Act
    finder.visit_file(&file);

    // Assert
    assert_eq!(
        (finder.unfollowed(), finder.found()),
        (vec![1, 2, 4], vec![included("followed.rs", 6, true, None)])
    );
}

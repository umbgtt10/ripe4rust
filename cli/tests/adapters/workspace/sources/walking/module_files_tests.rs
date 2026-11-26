// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::sources::walking::declared_module::DeclaredModule;
use cargo_ripe4rust::adapters::workspace::sources::walking::module_files::ModuleFiles;
use std::path::Path;
use std::path::PathBuf;

fn module(name: &str, inline: &[&str], path: Option<&str>) -> DeclaredModule {
    DeclaredModule {
        name: String::from(name),
        inline: inline.iter().map(|name| String::from(*name)).collect(),
        path: path.map(String::from),
        public: false,
        line: 1,
    }
}

fn paths(paths: &[&str]) -> Vec<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[test]
fn candidates_of_a_module_declared_in_a_file_owning_its_directory_are_beside_it() {
    // Arrange & Act
    let candidates = ModuleFiles.candidates(
        Path::new("/work/ws/app/src/lib.rs"),
        true,
        &module("a", &[], None),
    );

    // Assert
    assert_eq!(
        candidates,
        paths(&["/work/ws/app/src/a.rs", "/work/ws/app/src/a/mod.rs"])
    );
}

#[test]
fn candidates_of_a_module_declared_in_another_file_are_under_its_name() {
    // Arrange & Act
    let candidates = ModuleFiles.candidates(
        Path::new("/work/ws/app/src/a.rs"),
        false,
        &module("b", &[], None),
    );

    // Assert
    assert_eq!(
        candidates,
        paths(&["/work/ws/app/src/a/b.rs", "/work/ws/app/src/a/b/mod.rs"])
    );
}

#[test]
fn candidates_of_a_module_inside_an_inline_one_are_under_the_inline_s_name() {
    // Arrange & Act
    let candidates = ModuleFiles.candidates(
        Path::new("/work/ws/app/src/lib.rs"),
        true,
        &module("d", &["c"], None),
    );

    // Assert
    assert_eq!(
        candidates,
        paths(&["/work/ws/app/src/c/d.rs", "/work/ws/app/src/c/d/mod.rs"])
    );
}

#[test]
fn candidates_of_a_path_are_beside_the_declaring_file_its_parents_resolved() {
    // Arrange & Act
    let candidates = ModuleFiles.candidates(
        Path::new("/work/ws/app/src/a.rs"),
        false,
        &module("io", &[], Some("../shared/io.rs")),
    );

    // Assert
    assert_eq!(candidates, paths(&["/work/ws/app/shared/io.rs"]));
}

#[test]
fn candidates_of_a_path_inside_an_inline_module_are_under_the_inline_s_name() {
    // Arrange & Act
    let candidates = ModuleFiles.candidates(
        Path::new("/work/ws/app/src/lib.rs"),
        true,
        &module("io", &["c"], Some("io.rs")),
    );

    // Assert
    assert_eq!(candidates, paths(&["/work/ws/app/src/c/io.rs"]));
}

#[test]
fn resolved_of_a_path_climbing_out_drops_what_it_climbs_out_of() {
    // Arrange & Act
    let resolved = ModuleFiles.resolved(Path::new("/work/ws/app/src/../shared/./io.rs"));

    // Assert
    assert_eq!(resolved, PathBuf::from("/work/ws/app/shared/io.rs"));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::dependency;
use crate::support::sources::member;
use cargo_ripe4rust::adapters::workspace::sources::naming::code_names::CodeNames;
use cargo_ripe4rust::system::model::declared_dependency::DeclaredDependency;

fn path_dependency(name: &str) -> DeclaredDependency {
    DeclaredDependency {
        name: String::from(name),
        rename: None,
        source: None,
    }
}

#[test]
fn known_of_a_path_dependency_and_of_the_member_itself_is_true() {
    // Arrange
    let names = CodeNames::new(&member(vec![path_dependency("app-core")]));

    // Act
    let known = [
        names.known("app_core"),
        names.known("app"),
        names.known("itoa"),
    ];

    // Assert
    assert_eq!(known, [true, true, false]);
}

#[test]
fn package_of_a_hyphenated_package_is_found_by_its_underscores() {
    // Arrange
    let names = CodeNames::new(&member(vec![dependency("smol-str", None)]));

    // Act
    let package = names.package("smol_str");

    // Assert
    assert_eq!(package, Some("smol-str"));
}

#[test]
fn package_of_a_path_dependency_is_none() {
    // Arrange
    let names = CodeNames::new(&member(vec![path_dependency("app-core")]));

    // Act
    let package = names.package("app_core");

    // Assert
    assert_eq!(package, None);
}

#[test]
fn package_of_a_renamed_dependency_is_found_by_its_new_name_only() {
    // Arrange
    let names = CodeNames::new(&member(vec![dependency("itoa", Some("fmt-int"))]));

    // Act
    let packages = [names.package("fmt_int"), names.package("itoa")];

    // Assert
    assert_eq!(packages, [Some("itoa"), None]);
}

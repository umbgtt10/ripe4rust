// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::cargo::metadata_reader::MetadataReader;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::requirement::Requirement;

const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";

fn internal(detail: &str) -> PortFailure {
    PortFailure::Internal {
        doing: String::from("reading what `cargo metadata` printed"),
        detail: String::from(detail),
    }
}

fn metadata(packages: &str, members: &str, nodes: &str) -> String {
    format!(
        r#"{{"packages":[{packages}],"workspace_members":[{members}],"resolve":{{"nodes":[{nodes}]}}}}"#
    )
}

fn package(name: &str, version: &str, source: Option<&str>, dependencies: &str) -> String {
    let id = format!("{name}@{version}");
    let source = source.map_or_else(|| String::from("null"), |source| format!("\"{source}\""));
    format!(
        r#"{{"name":"{name}","version":"{version}","id":"{id}","source":{source},"rust_version":null,"dependencies":[{dependencies}]}}"#
    )
}

#[test]
fn registry_crates_of_the_recorded_filter_leave_out_the_unix_only_crates() {
    // Arrange & Act
    let names = MetadataReader.registry_crates(include_str!(
        "../../fixtures/workspace/cargo/metadata-thumbv7em-none-eabihf.json"
    ));

    // Assert
    assert_eq!(
        names,
        Ok(vec![
            String::from("either"),
            String::from("itertools"),
            String::from("smallvec"),
        ])
    );
}

#[test]
fn workspace_of_a_crate_declared_twice_takes_the_requirement_its_version_meets() {
    // Arrange
    let declared = [
        format!(r#"{{"name":"widget","req":"^2","source":"{CRATES_IO}"}}"#),
        format!(r#"{{"name":"widget","req":"^1.4","source":"{CRATES_IO}"}}"#),
    ]
    .join(",");
    let text = metadata(
        &[
            package("app", "0.1.0", None, &declared),
            package("widget", "1.4.0", Some(CRATES_IO), ""),
        ]
        .join(","),
        r#""app@0.1.0""#,
        r#"{"id":"app@0.1.0","deps":[{"pkg":"widget@1.4.0"}]},{"id":"widget@1.4.0","deps":[]}"#,
    );

    // Act
    let workspace = MetadataReader.workspace(&text).expect("a workspace");

    // Assert
    assert_eq!(
        workspace.members[0].dependencies[0].requirement,
        Requirement::parse("^1.4").expect("a requirement")
    );
}

#[test]
fn workspace_of_a_crate_from_another_registry_leaves_it_out() {
    // Arrange
    let text = metadata(
        &[
            package(
                "app",
                "0.1.0",
                None,
                r#"{"name":"private","req":"^1","source":"registry+https://example.org/index"}"#,
            ),
            package(
                "private",
                "1.0.0",
                Some("registry+https://example.org/index"),
                "",
            ),
        ]
        .join(","),
        r#""app@0.1.0""#,
        r#"{"id":"app@0.1.0","deps":[{"pkg":"private@1.0.0"}]},{"id":"private@1.0.0","deps":[]}"#,
    );

    // Act
    let workspace = MetadataReader.workspace(&text).expect("a workspace");

    // Assert
    assert_eq!(workspace.lock, Vec::new());
    assert_eq!(workspace.members[0].dependencies, Vec::new());
}

#[test]
fn workspace_of_a_member_the_resolve_does_not_hold_is_internal() {
    // Arrange
    let text = metadata(&package("app", "0.1.0", None, ""), r#""app@0.1.0""#, "");

    // Act
    let workspace = MetadataReader.workspace(&text);

    // Assert
    assert_eq!(
        workspace,
        Err(internal("`app@0.1.0` is not in the resolve"))
    );
}

#[test]
fn workspace_of_a_version_that_is_none_is_internal() {
    // Arrange
    let text = metadata(
        &package("widget", "one", Some(CRATES_IO), ""),
        "",
        r#"{"id":"widget@one","deps":[]}"#,
    );

    // Act
    let workspace = MetadataReader.workspace(&text);

    // Assert
    assert_eq!(workspace, Err(internal("`one` is not a version")));
}

#[test]
fn workspace_without_a_resolve_is_internal() {
    // Arrange
    let text = r#"{"packages":[],"workspace_members":[],"resolve":null}"#;

    // Act
    let workspace = MetadataReader.workspace(text);

    // Assert
    assert_eq!(workspace, Err(internal("it holds no resolve")));
}

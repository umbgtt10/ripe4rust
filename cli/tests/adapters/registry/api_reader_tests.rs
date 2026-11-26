// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::registry::api_reader::ApiReader;
use ripe4rust_core::inputs::port_failure::PortFailure;

#[test]
fn repository_of_a_crate_naming_none_is_none() {
    // Arrange & Act
    let repository =
        ApiReader.repository("widget", r#"{"crate":{"name":"widget","repository":null}}"#);

    // Assert
    assert_eq!(repository, Ok(None));
}

#[test]
fn repository_of_an_answer_that_is_not_json_is_internal() {
    // Arrange & Act
    let repository = ApiReader.repository("widget", "<html>");

    // Assert
    assert!(
        matches!(&repository, Err(PortFailure::Internal { doing, .. }) if doing == "reading the API's answer for `widget`"),
        "{repository:?}"
    );
}

#[test]
fn repository_of_an_empty_answer_is_none() {
    // Arrange & Act
    let repository = ApiReader.repository("widget", "");

    // Assert
    assert_eq!(repository, Ok(None));
}

#[test]
fn repository_of_the_recorded_answer_is_its_repository() {
    // Arrange
    let recorded = include_str!("../../fixtures/workspace/registry/api/itoa.txt");
    let (body, _) = recorded.rsplit_once('\n').expect("a status");

    // Act
    let repository = ApiReader.repository("itoa", body);

    // Assert
    assert_eq!(
        repository,
        Ok(Some(String::from("https://github.com/dtolnay/itoa")))
    );
}

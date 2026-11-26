// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::exposure_override::ExposureOverride;
use ripe4rust_core::planning::pins_and_policy::exposure_reader::ExposureReader;

#[test]
fn read_of_an_exposure_neither_public_nor_private_is_invalid() {
    // Arrange
    let entries = [entry(
        "exposure",
        PolicyValue::Table(vec![entry("serde", text("exported"), 35)]),
        35,
    )];

    // Act
    let reading = ExposureReader.read(&entries);

    // Assert
    assert_eq!(reading.value, []);
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.what.clone())
            .collect::<Vec<_>>(),
        vec![String::from(
            "`serde` in `exposure` is not `public` or `private`"
        )]
    );
}

#[test]
fn read_of_an_exposure_that_is_not_a_table_is_invalid() {
    // Arrange
    let entries = [entry("exposure", text("serde"), 35)];

    // Act
    let reading = ExposureReader.read(&entries);

    // Assert
    assert_eq!(
        reading
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>(),
        vec![DiagnosticCode::PolicyInvalidValue]
    );
}

#[test]
fn read_of_no_exposure_overrides_nothing() {
    // Arrange & Act
    let reading = ExposureReader.read(&[]);

    // Assert
    assert_eq!(reading.value, []);
}

#[test]
fn read_of_public_and_private_overrides_each_crate() {
    // Arrange
    let entries = [entry(
        "exposure",
        PolicyValue::Table(vec![
            entry("serde", text("public"), 35),
            entry("anyhow", text("private"), 35),
        ]),
        35,
    )];

    // Act
    let reading = ExposureReader.read(&entries);

    // Assert
    assert_eq!(
        reading.value,
        vec![
            ExposureOverride {
                name: String::from("serde"),
                public: true,
            },
            ExposureOverride {
                name: String::from("anyhow"),
                public: false,
            },
        ]
    );
}

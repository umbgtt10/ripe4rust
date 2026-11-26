// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::unit_kind::UnitKind;

#[test]
fn label_of_each_kind_is_its_name() {
    // Arrange & Act
    let labels = [
        UnitKind::Advisory,
        UnitKind::Breaking,
        UnitKind::Compatible,
        UnitKind::Git,
        UnitKind::Toolchain,
    ]
    .map(|kind| kind.label());

    // Assert
    assert_eq!(
        labels,
        ["advisory", "breaking", "compatible", "git", "toolchain"]
    );
}

#[test]
fn parse_of_each_label_is_its_kind_and_of_anything_else_none() {
    // Arrange & Act
    let parsed = [
        UnitKind::parse("advisory"),
        UnitKind::parse("breaking"),
        UnitKind::parse("compatible"),
        UnitKind::parse("git"),
        UnitKind::parse("toolchain"),
        UnitKind::parse("major"),
    ];

    // Assert
    assert_eq!(
        parsed,
        [
            Some(UnitKind::Advisory),
            Some(UnitKind::Breaking),
            Some(UnitKind::Compatible),
            Some(UnitKind::Git),
            Some(UnitKind::Toolchain),
            None,
        ]
    );
}

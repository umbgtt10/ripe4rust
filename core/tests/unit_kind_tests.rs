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
        UnitKind::Toolchain,
    ]
    .map(|kind| kind.label());

    // Assert
    assert_eq!(labels, ["advisory", "breaking", "compatible", "toolchain"]);
}

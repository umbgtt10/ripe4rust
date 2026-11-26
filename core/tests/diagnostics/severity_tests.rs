// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::diagnostics::severity::Severity;

#[test]
fn label_of_each_severity_is_its_name() {
    // Arrange & Act
    let labels =
        [Severity::Error, Severity::Warning, Severity::Notice].map(|severity| severity.label());

    // Assert
    assert_eq!(labels, ["error", "warning", "notice"]);
}

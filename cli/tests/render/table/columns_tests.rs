// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::render::table::columns::Columns;

#[test]
fn lines_widen_a_column_past_its_minimum_for_a_long_cell() {
    // Arrange
    let rows = [
        ["unit", "kind", "from", "to", "outcome", "reasons"].map(String::from),
        [
            "a-very-long-crate-name-indeed",
            "breaking",
            "1.0.0",
            "2.0.0",
            "ask",
            "why",
        ]
        .map(String::from),
    ];

    // Act
    let lines = Columns.lines(&rows);

    // Assert
    assert_eq!(
        lines,
        vec![
            String::from(
                "unit                           kind        from      to        outcome          reasons"
            ),
            String::from(
                "a-very-long-crate-name-indeed  breaking    1.0.0     2.0.0     ask              why"
            ),
        ]
    );
}

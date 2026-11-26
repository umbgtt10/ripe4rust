// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::wording::Wording;

#[test]
fn count_of_one_is_singular() {
    // Arrange & Act
    let counted = Wording.count(1, "release");

    // Assert
    assert_eq!(counted, "1 release");
}

#[test]
fn count_of_several_is_plural() {
    // Arrange & Act
    let counted = Wording.count(42, "line");

    // Assert
    assert_eq!(counted, "42 lines");
}

#[test]
fn footprint_of_a_complete_parse_counts_its_lines_and_files() {
    // Arrange & Act
    let said = Wording.footprint(Footprint {
        lines: 9,
        files: 2,
        complete: true,
    });

    // Assert
    assert_eq!(said, "footprint 9 lines in 2 files");
}

#[test]
fn footprint_of_a_parse_missing_included_code_is_a_floor() {
    // Arrange & Act
    let said = [(3, 1), (0, 0)].map(|(lines, files)| {
        Wording.footprint(Footprint {
            lines,
            files,
            complete: false,
        })
    });

    // Assert
    assert_eq!(
        said,
        [
            String::from(
                "footprint at least 3 lines in 1 file: code is included the parse cannot follow"
            ),
            String::from("named nowhere the parse could read: code is included it cannot follow"),
        ]
    );
}

#[test]
fn list_joins_with_commas() {
    // Arrange
    let items = [
        String::from("RUSTSEC-2026-0190"),
        String::from("RUSTSEC-2026-0191"),
    ];

    // Act
    let listed = Wording.list(&items);

    // Assert
    assert_eq!(listed, "RUSTSEC-2026-0190, RUSTSEC-2026-0191");
}

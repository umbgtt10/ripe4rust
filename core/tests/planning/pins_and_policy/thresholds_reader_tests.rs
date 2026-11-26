// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::policy_trees::entry;
use crate::support::policy_trees::text;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::inputs::policy_value::PolicyValue;
use ripe4rust_core::planning::pins_and_policy::thresholds::Thresholds;
use ripe4rust_core::planning::pins_and_policy::thresholds_reader::ThresholdsReader;

#[test]
fn read_of_a_threshold_in_words_keeps_its_default_and_says_so() {
    // Arrange
    let entries = [entry("stale-months", text("six"), 34)];

    // Act
    let reading = ThresholdsReader.read(&entries);

    // Assert
    assert_eq!(reading.value.stale_months, 6);
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
fn read_of_every_threshold_sets_each() {
    // Arrange
    let entries = [
        entry("young-days", PolicyValue::Integer(1), 31),
        entry("young-releases", PolicyValue::Integer(2), 32),
        entry("toolchain-young-days", PolicyValue::Integer(3), 33),
        entry("stale-months", PolicyValue::Integer(4), 34),
        entry("small-footprint", PolicyValue::Integer(5), 35),
        entry("moderate-footprint", PolicyValue::Integer(6), 36),
        entry("hold-days", PolicyValue::Integer(7), 37),
    ];

    // Act
    let reading = ThresholdsReader.read(&entries);

    // Assert
    assert_eq!(
        reading.value,
        Thresholds {
            young_days: 1,
            young_releases: 2,
            toolchain_young_days: 3,
            stale_months: 4,
            small_footprint: 5,
            moderate_footprint: 6,
            hold_days: 7,
        }
    );
}

#[test]
fn read_of_no_threshold_takes_every_default() {
    // Arrange & Act
    let reading = ThresholdsReader.read(&[]);

    // Assert
    assert_eq!(
        reading.value,
        Thresholds {
            young_days: 30,
            young_releases: 3,
            toolchain_young_days: 0,
            stale_months: 6,
            small_footprint: 10,
            moderate_footprint: 100,
            hold_days: 14,
        }
    );
    assert_eq!(reading.diagnostics, []);
}

#[test]
fn read_passes_over_keys_that_are_not_thresholds() {
    // Arrange
    let entries = [entry("gates", text("just stage1"), 31)];

    // Act
    let reading = ThresholdsReader.read(&entries);

    // Assert
    assert_eq!(reading.diagnostics, []);
}

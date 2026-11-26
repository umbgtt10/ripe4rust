// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::clock::system_clock::SystemClock;
use ripe4rust_core::values::date::Date;
use std::time::Duration;

#[test]
fn day_of_a_moment_before_midnight_is_that_day() {
    // Arrange & Act
    let day = SystemClock.day_of(Duration::from_secs(20_783 * 86_400 + 86_399));

    // Assert
    assert_eq!(day, Date::parse("2026-11-26").expect("a date"));
}

#[test]
fn day_of_midnight_is_the_new_day() {
    // Arrange & Act
    let day = SystemClock.day_of(Duration::from_secs(20_784 * 86_400));

    // Assert
    assert_eq!(day, Date::parse("2026-11-27").expect("a date"));
}

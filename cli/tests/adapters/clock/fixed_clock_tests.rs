// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::clock::fixed_clock::FixedClock;
use ripe4rust_core::traits::clock::Clock;
use ripe4rust_core::values::date::Date;

#[test]
fn today_is_the_date_it_was_given() {
    // Arrange
    let date = Date::parse("2026-10-30").expect("a date");

    // Act
    let today = FixedClock::new(date).today();

    // Assert
    assert_eq!(today, date);
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::parse_failure::ParseFailure;

fn date(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

#[test]
fn add_days_counts_a_leap_day() {
    // Arrange & Act
    let later = date("2028-02-28").add_days(1);

    // Assert
    assert_eq!(later, date("2028-02-29"));
}

#[test]
fn add_days_crosses_a_month_and_a_year() {
    // Arrange & Act
    let later = date("2026-12-20").add_days(14);

    // Assert
    assert_eq!(later, date("2027-01-03"));
}

#[test]
fn add_days_of_a_negative_count_goes_back() {
    // Arrange & Act
    let earlier = date("2026-03-01").add_days(-1);

    // Assert
    assert_eq!(earlier, date("2026-02-28"));
}

#[test]
fn cmp_orders_by_year_month_then_day() {
    // Arrange
    let earlier = date("2026-10-30");

    // Act
    let later = date("2026-11-02");

    // Assert
    assert!(earlier < later);
}

#[test]
fn days_since_a_later_date_is_negative() {
    // Arrange
    let later = date("2027-01-01");

    // Act
    let days = date("2026-12-31").days_since(&later);

    // Assert
    assert_eq!(days, -1);
}

#[test]
fn days_since_counts_the_days_between() {
    // Arrange
    let earlier = date("2026-10-02");

    // Act
    let days = date("2026-11-26").days_since(&earlier);

    // Assert
    assert_eq!(days, 55);
}

#[test]
fn fmt_pads_month_and_day() {
    // Arrange & Act
    let text = date("2027-01-05").to_string();

    // Assert
    assert_eq!(text, "2027-01-05");
}

#[test]
fn from_unix_days_of_a_day_before_the_epoch_is_in_1969() {
    // Arrange & Act
    let date = Date::from_unix_days(-1);

    // Assert
    assert_eq!(date.to_string(), "1969-12-31");
}

#[test]
fn from_unix_days_of_the_days_since_the_epoch_is_that_day() {
    // Arrange & Act
    let date = Date::from_unix_days(20_783);

    // Assert
    assert_eq!(date, Date::parse("2026-11-26").expect("a date"));
}

#[test]
fn from_unix_days_of_zero_is_the_epoch() {
    // Arrange & Act
    let date = Date::from_unix_days(0);

    // Assert
    assert_eq!(date.to_string(), "1970-01-01");
}

#[test]
fn months_before_clamps_to_the_last_day_of_a_shorter_month() {
    // Arrange & Act
    let earlier = date("2026-08-31").months_before(6);

    // Assert
    assert_eq!(earlier, date("2026-02-28"));
}

#[test]
fn months_before_crosses_years() {
    // Arrange & Act
    let earlier = date("2026-03-15").months_before(15);

    // Assert
    assert_eq!(earlier, date("2024-12-15"));
}

#[test]
fn months_before_keeps_the_day() {
    // Arrange & Act
    let earlier = date("2026-11-26").months_before(6);

    // Assert
    assert_eq!(earlier, date("2026-05-26"));
}

#[test]
fn parse_of_a_day_past_the_month_fails() {
    // Arrange & Act
    let parsed = Date::parse("2026-02-29");

    // Assert
    assert_eq!(
        parsed,
        Err(ParseFailure {
            text: String::from("2026-02-29"),
            expected: "a date, YYYY-MM-DD",
        })
    );
}

#[test]
fn parse_of_a_day_zero_fails() {
    // Arrange & Act
    let parsed = Date::parse("2026-10-00");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_a_leap_day_in_a_century_fails() {
    // Arrange & Act
    let parsed = Date::parse("2100-02-29");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_a_leap_day_in_a_leap_year_succeeds() {
    // Arrange & Act
    let parsed = Date::parse("2000-02-29");

    // Assert
    assert!(parsed.is_ok());
}

#[test]
fn parse_of_a_month_thirteen_fails() {
    // Arrange & Act
    let parsed = Date::parse("2026-13-01");

    // Assert
    assert!(parsed.is_err());
}

#[test]
fn parse_of_a_timestamp_fails() {
    // Arrange & Act
    let parsed = Date::parse("2026-10-02T12:00:00Z");

    // Assert
    assert!(parsed.is_err());
}

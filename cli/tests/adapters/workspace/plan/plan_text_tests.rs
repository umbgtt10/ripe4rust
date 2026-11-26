// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::plan::plan_text::PlanText;
use ripe4rust_core::inputs::lock_entry::LockEntry;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::location::Location;
use ripe4rust_core::values::version::Version;

fn unreadable(detail: &str) -> PortFailure {
    PortFailure::PlanUnreadable {
        file: String::from("plan.json"),
        detail: String::from(detail),
    }
}

#[test]
fn date_of_a_date_is_it_and_of_anything_else_unreadable() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = [text.date("2026-11-26"), text.date("26 November")];

    // Assert
    assert_eq!(
        read,
        [
            Ok(Date::parse("2026-11-26").expect("a date")),
            Err(unreadable("`26 November` is not a date, YYYY-MM-DD")),
        ]
    );
}

#[test]
fn entry_of_a_crate_and_its_version_is_the_lock_s_entry() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = text.entry("itoa 0.4.7");

    // Assert
    assert_eq!(
        read,
        Ok(LockEntry {
            name: String::from("itoa"),
            version: Version::parse("0.4.7").expect("a version"),
        })
    );
}

#[test]
fn location_of_a_file_and_line_is_it_and_of_anything_else_unreadable() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = [
        text.location("app/Cargo.toml:12"),
        text.location("Cargo.toml"),
        text.location(":4"),
    ];

    // Assert
    assert_eq!(
        read,
        [
            Ok(Location {
                file: String::from("app/Cargo.toml"),
                line: 12,
            }),
            Err(unreadable(
                "`Cargo.toml` is not a file and a line, file:line"
            )),
            Err(unreadable("`:4` is not a file and a line, file:line")),
        ]
    );
}

#[test]
fn optional_version_of_none_is_none_and_of_text_its_version() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = [
        text.optional_version(None),
        text.optional_version(Some("1.4.1")),
    ];

    // Assert
    assert_eq!(
        read,
        [
            Ok(None),
            Ok(Some(Version::parse("1.4.1").expect("a version")))
        ]
    );
}

#[test]
fn unreadable_names_the_plan_and_what_is_wrong() {
    // Arrange & Act
    let failure = PlanText::new("plans/today.json").unreadable("it is empty");

    // Assert
    assert_eq!(
        failure,
        PortFailure::PlanUnreadable {
            file: String::from("plans/today.json"),
            detail: String::from("it is empty"),
        }
    );
}

#[test]
fn version_of_text_is_its_version_and_of_anything_else_unreadable() {
    // Arrange
    let text = PlanText::new("plan.json");

    // Act
    let read = [text.version("1.4.1"), text.version("x")];

    // Assert
    assert_eq!(
        read,
        [
            Ok(Version::parse("1.4.1").expect("a version")),
            Err(unreadable("`x` is not a version")),
        ]
    );
}

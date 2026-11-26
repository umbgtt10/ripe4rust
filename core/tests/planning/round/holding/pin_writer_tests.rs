// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use crate::support::surveyed::at;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::pins_and_policy::release_condition::ReleaseCondition;
use ripe4rust_core::planning::round::holding::held_crate::HeldCrate;
use ripe4rust_core::planning::round::holding::pin_writer::PinWriter;
use ripe4rust_core::values::requirement::Requirement;

fn held(name: &str, hold: Hold) -> HeldCrate {
    HeldCrate {
        name: String::from(name),
        hold,
    }
}

fn pinned(review: Option<&str>, until: Option<ReleaseCondition>, hold: Hold) -> Pin {
    Pin {
        name: String::from("gizmo"),
        reason: String::from("3 is a migration"),
        review: review.map(date),
        until,
        hold,
        at: at("Cargo.toml", 41),
    }
}

#[test]
fn line_of_a_crate_already_pinned_keeps_its_reason_condition_and_earlier_review() {
    // Arrange
    let writer = PinWriter::new("2.7.6 broke stage 2", date("2026-12-10"));
    let until = ReleaseCondition::Release {
        name: String::from("gizmo-cli"),
        requirement: Requirement::parse(">=0.32").expect("a requirement"),
    };

    // Act
    let line = writer.line(
        &held("gizmo", Hold::All),
        Some(&pinned(Some("2027-03-01"), Some(until), Hold::Line)),
    );

    // Assert
    assert_eq!(
        line,
        r#"gizmo = { reason = "3 is a migration; 2.7.6 broke stage 2", review = "2026-12-10", until = { crate = "gizmo-cli", version = ">=0.32" }, hold = "all" }"#
    );
}

#[test]
fn line_of_a_crate_held_on_its_line_names_no_hold() {
    // Arrange
    let writer = PinWriter::new("the person: plan it for January", date("2027-01-31"));

    // Act
    let line = writer.line(&held("gadget", Hold::Line), None);

    // Assert
    assert_eq!(
        line,
        r#"gadget = { reason = "the person: plan it for January", review = "2027-01-31" }"#
    );
}

#[test]
fn line_of_a_crate_held_whole_says_so_and_escapes_its_reason() {
    // Arrange
    let writer = PinWriter::new(
        r#"stage 2: "crap4rust" over 15 in C:\ws"#,
        date("2026-12-10"),
    );

    // Act
    let line = writer.line(&held("widget", Hold::All), None);

    // Assert
    assert_eq!(
        line,
        r#"widget = { reason = "stage 2: \"crap4rust\" over 15 in C:\\ws", review = "2026-12-10", hold = "all" }"#
    );
}

#[test]
fn line_of_a_pin_in_words_keeps_its_words_and_a_review_sooner_than_the_new() {
    // Arrange
    let writer = PinWriter::new("2.7.6 broke stage 2", date("2026-12-10"));

    // Act
    let line = writer.line(
        &held("gizmo", Hold::Line),
        Some(&pinned(
            Some("2026-12-01"),
            Some(ReleaseCondition::Words(String::from(
                "the load test passes",
            ))),
            Hold::All,
        )),
    );

    // Assert
    assert_eq!(
        line,
        r#"gizmo = { reason = "3 is a migration; 2.7.6 broke stage 2", review = "2026-12-01", until = "the load test passes", hold = "all" }"#
    );
}

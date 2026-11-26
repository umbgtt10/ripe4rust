// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::surveyed::at;
use ripe4rust_core::planning::round::holding::pin_line::PinLine;
use ripe4rust_core::planning::round::holding::pin_placement::PinPlacement;

fn pin(name: &str, replaces: Option<u32>) -> PinLine {
    PinLine {
        name: String::from(name),
        line: format!("{name} = {{ reason = \"r\", review = \"2026-12-10\" }}"),
        replaces: replaces.map(|line| at("Cargo.toml", line)),
    }
}

#[test]
fn action_of_a_pin_replacing_one_says_which() {
    // Arrange & Act
    let action = PinPlacement.action(Some(&at("Cargo.toml", 40)), &[pin("gizmo", Some(41))]);

    // Assert
    assert_eq!(
        action,
        "put the pin under `[workspace.metadata.ripe4rust.pins]` at Cargo.toml:40, in place of the pin on `gizmo` at Cargo.toml:41, and commit it"
    );
}

#[test]
fn action_of_a_pin_with_its_table_puts_it_under_it() {
    // Arrange & Act
    let action = PinPlacement.action(Some(&at("Cargo.toml", 40)), &[pin("widget", None)]);

    // Assert
    assert_eq!(
        action,
        "put the pin under `[workspace.metadata.ripe4rust.pins]` at Cargo.toml:40, and commit it"
    );
}

#[test]
fn action_of_pins_with_no_table_adds_the_table_at_the_end() {
    // Arrange & Act
    let action = PinPlacement.action(None, &[pin("widget", None), pin("gizmo", None)]);

    // Assert
    assert_eq!(
        action,
        "add `[workspace.metadata.ripe4rust.pins]` at the end of `Cargo.toml`, put the pins under it, and commit it"
    );
}

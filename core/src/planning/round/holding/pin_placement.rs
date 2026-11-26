// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::round::holding::pin_line::PinLine;
use crate::values::location::Location;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Where the pins `hold` prints go, and what to do with them: under the
/// manifest's pins table, in place of a pin the crate has, or in a table
/// added at the end of the workspace's `Cargo.toml`.
pub struct PinPlacement;

impl PinPlacement {
    pub fn action(&self, table: Option<&Location>, pins: &[PinLine]) -> String {
        let them = if pins.len() == 1 {
            "the pin"
        } else {
            "the pins"
        };
        let Some(table) = table else {
            return format!(
                "add `[workspace.metadata.ripe4rust.pins]` at the end of `Cargo.toml`, put {them} under it, and commit it"
            );
        };
        let replaced = pins
            .iter()
            .filter_map(|pin| {
                pin.replaces
                    .as_ref()
                    .map(|at| format!(", in place of the pin on `{}` at {at}", pin.name))
            })
            .collect::<Vec<String>>()
            .concat();
        format!(
            "put {them} under `[workspace.metadata.ripe4rust.pins]` at {table}{replaced}, and commit it"
        )
    }
}

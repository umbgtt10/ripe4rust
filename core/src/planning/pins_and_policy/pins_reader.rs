// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_entry::PolicyEntry;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::pins_and_policy::pin_reader::PinReader;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use alloc::vec;
use alloc::vec::Vec;

const SECTION: &str = "`[workspace.metadata.ripe4rust]`";

/// `[workspace.metadata.ripe4rust.pins]`, every pin read, the refused ones
/// reported.
pub struct PinsReader;

impl PinsReader {
    pub fn read(&self, entries: &[PolicyEntry]) -> Reading<Vec<Pin>> {
        let mut read = Reading {
            value: Vec::new(),
            diagnostics: Vec::new(),
        };
        let Some(pins) = entries.iter().find(|entry| entry.key == "pins") else {
            return read;
        };
        match PolicyValues::new(SECTION).table(pins) {
            Ok(each) => {
                for pin in each.iter().map(|pin| PinReader.read(pin)) {
                    read.value.extend(pin.value);
                    read.diagnostics.extend(pin.diagnostics);
                }
                read
            }
            Err(diagnostic) => Reading {
                value: Vec::new(),
                diagnostics: vec![diagnostic],
            },
        }
    }
}

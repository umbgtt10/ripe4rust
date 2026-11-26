// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// One pin `hold` prints: the crate, the line to write under the pins
/// table, and the pin on the crate it takes the place of, if one is there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinLine {
    pub name: String,
    pub line: String,
    pub replaces: Option<Location>,
}

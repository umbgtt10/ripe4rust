// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::hold::Hold;
use alloc::string::String;

/// A crate a pin is to hold, and what of it: its new line, or every move.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeldCrate {
    pub name: String,
    pub hold: Hold,
}

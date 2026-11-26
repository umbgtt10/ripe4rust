// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::pin::Pin;
use alloc::string::String;

/// A pin holding what a unit moves, and what released it, if anything has:
/// its review date reached, or its condition's release out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinStanding {
    pub pin: Pin,
    pub released: Option<String>,
}

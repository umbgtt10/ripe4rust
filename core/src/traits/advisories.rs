// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::advisory::Advisory;
use crate::inputs::port_failure::PortFailure;
use alloc::vec::Vec;

/// The advisories against the versions in use, from cargo-deny.
pub trait Advisories {
    fn advisories(&self) -> Result<Vec<Advisory>, PortFailure>;
}

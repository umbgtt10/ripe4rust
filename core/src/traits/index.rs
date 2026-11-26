// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::release::Release;
use crate::values::date::Date;
use alloc::vec::Vec;

/// The registry's sparse index: every release of a crate, with its yanked
/// flag, `rust-version` and date, as read on `read_on`.
pub trait Index {
    fn read_on(&self) -> Date;
    fn releases(&self, name: &str) -> Result<Vec<Release>, PortFailure>;
}

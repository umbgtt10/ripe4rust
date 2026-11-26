// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::release::Release;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec::Vec;

/// The registry's sparse index: every release of each crate asked for, in
/// the order asked, with its yanked flag, `rust-version` and date, as read on
/// `read_on`. One question for every crate, so the answers can be fetched
/// together.
pub trait Index {
    fn read_on(&self) -> Date;
    fn releases(&self, names: &[String]) -> Result<Vec<(String, Vec<Release>)>, PortFailure>;
}

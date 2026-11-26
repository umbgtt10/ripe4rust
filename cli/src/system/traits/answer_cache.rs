// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;

/// The registry's answers kept for the day they were read: an answer of a
/// kind -- the index's, the API's -- for a crate, `None` where it is to be
/// fetched, and the day the answers read are from.
pub trait AnswerCache {
    fn read(&self, kind: &str, name: &str) -> Result<Option<String>, PortFailure>;
    fn read_on(&self) -> Date;
    fn write(&self, kind: &str, name: &str, body: &str) -> Result<(), PortFailure>;
}

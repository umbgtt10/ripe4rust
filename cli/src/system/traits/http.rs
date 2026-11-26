// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::fetched::Fetched;
use ripe4rust_core::inputs::port_failure::PortFailure;

/// An HTTP GET: the answer, whatever its status, or why there was none; and
/// several, fetched together, answered in their order.
pub trait Http {
    fn get(&self, url: &str) -> Result<Fetched, PortFailure>;
    fn get_all(&self, urls: &[String]) -> Vec<Result<Fetched, PortFailure>>;
}

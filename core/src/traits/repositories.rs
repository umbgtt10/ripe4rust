// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use alloc::string::String;

/// A direct dependency's repository, from the registry's API, for grouping.
pub trait Repositories {
    fn repository(&self, name: &str) -> Result<Option<String>, PortFailure>;
}

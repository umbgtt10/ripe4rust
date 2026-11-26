// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// `rust-toolchain.toml`'s channel, as written, and its line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolchainFile {
    pub channel: String,
    pub at: Location,
}

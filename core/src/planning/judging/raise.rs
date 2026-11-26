// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::partial_version::PartialVersion;
use alloc::string::String;

/// A member's `rust-version` a breaking unit's target needs raised.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raise {
    pub member: String,
    pub to: PartialVersion,
    pub published: bool,
}

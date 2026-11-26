// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// A line a unit's `apply` can edit, and what is written there: a
/// dependency's requirement, `key` the crate; a member's `rust-version`; or
/// the toolchain's `channel`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrittenLine {
    pub at: Location,
    pub key: String,
    pub value: String,
}

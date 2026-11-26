// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;

/// A move inside a unit to a lower version of a crate, and whether the
/// version it leaves is yanked -- the move back then being the cure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Downgrade {
    pub name: String,
    pub from: Version,
    pub to: Version,
    pub yanked: bool,
}

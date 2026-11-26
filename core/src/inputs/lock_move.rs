// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;
use alloc::string::String;

/// One line of `cargo update --dry-run`: a crate updated, added (no
/// `from`) or removed (no `to`) in the lock file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockMove {
    pub name: String,
    pub from: Option<Version>,
    pub to: Option<Version>,
}

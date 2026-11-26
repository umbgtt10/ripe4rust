// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

/// How to take a unit back: the files to restore, and the command that does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Revert {
    pub restore: Vec<String>,
    pub run: String,
}

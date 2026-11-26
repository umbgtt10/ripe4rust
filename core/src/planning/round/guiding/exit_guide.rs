// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A code a command exits with, and what it means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExitGuide {
    pub code: u8,
    pub means: String,
}

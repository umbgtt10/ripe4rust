// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// What a command takes, as its usage writes it, and what it means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptionGuide {
    pub option: String,
    pub means: String,
}

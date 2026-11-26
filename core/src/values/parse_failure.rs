// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A text that does not read as the value it should be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseFailure {
    pub text: String,
    pub expected: &'static str,
}

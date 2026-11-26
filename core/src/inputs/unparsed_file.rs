// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A source file the parse could not read, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnparsedFile {
    pub file: String,
    pub detail: String,
}

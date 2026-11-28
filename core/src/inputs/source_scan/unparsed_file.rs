// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// A source file the parse could not read, where to look, and why: the line
/// its parse stopped at, its first line where it could not be read at all,
/// or the `mod` item declaring it where it is not there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnparsedFile {
    pub file: String,
    pub at: Location,
    pub detail: String,
}

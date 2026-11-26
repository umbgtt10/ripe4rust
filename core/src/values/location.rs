// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// A line of a workspace's file, written `file:line`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub file: String,
    pub line: u32,
}

impl Display for Location {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}:{}", self.file, self.line)
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Every release of one major, or of one minor where the major is 0 and for
/// the toolchain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub major: u64,
    pub minor: Option<u64>,
}

impl Display for Line {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self.minor {
            Some(minor) => write!(formatter, "{}.{minor}.x", self.major),
            None => write!(formatter, "{}.x", self.major),
        }
    }
}

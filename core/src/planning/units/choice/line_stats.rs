// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::date::Date;
use crate::values::line::Line;

/// A line's releases that count -- neither yanked nor pre-releases -- and the
/// dates of the first and the last of them that the index dates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineStats {
    pub line: Line,
    pub first_release: Option<Date>,
    pub last_release: Option<Date>,
    pub releases: u32,
}

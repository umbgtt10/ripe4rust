// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::written_line::WrittenLine;

/// A line a unit can edit that reads otherwise than the plan found it:
/// gone, new, or rewritten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineChange {
    Gone { then: WrittenLine },
    New { now: WrittenLine },
    Rewritten { then: WrittenLine, now: WrittenLine },
}

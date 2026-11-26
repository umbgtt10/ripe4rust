// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What a pin holds: by default the move to a new line, or every move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hold {
    All,
    Line,
}

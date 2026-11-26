// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// One thing to do: a command to run, or a value to write at a line of a
/// workspace file -- a requirement, a `rust-version`, the channel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    Edit {
        at: Location,
        key: String,
        value: String,
    },
    Run {
        command: String,
    },
}

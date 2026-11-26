// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::round::guiding::exit_guide::ExitGuide;
use crate::planning::round::guiding::option_guide::OptionGuide;
use alloc::string::String;
use alloc::vec::Vec;

/// One command of ripe4rust: its usage, what it does, what it takes, and
/// the codes it exits with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandGuide {
    pub usage: String,
    pub does: String,
    pub options: Vec<OptionGuide>,
    pub exits: Vec<ExitGuide>,
}

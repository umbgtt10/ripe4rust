// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::requirement::Requirement;
use alloc::string::String;

/// What releases a pin: a crate reaching a version, checked against the
/// index, or words, never evaluated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReleaseCondition {
    Release {
        name: String,
        requirement: Requirement,
    },
    Words(String),
}

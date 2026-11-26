// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::writing::step::Step;
use crate::values::location::Location;
use alloc::string::String;

/// The edits a unit's `apply` names, each a value written for a key at a
/// line: a breaking unit's requirement, written as its target version, a
/// caret requirement; the `rust-version` its target needs; the toolchain's
/// channel.
pub struct Edits;

impl Edits {
    pub fn set(&self, at: Location, key: &str, value: String) -> Step {
        Step::Edit {
            at,
            key: String::from(key),
            value,
        }
    }
}

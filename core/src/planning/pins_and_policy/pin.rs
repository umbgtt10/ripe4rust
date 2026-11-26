// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::values::date::Date;
use crate::values::location::Location;
use alloc::string::String;

/// A dependency held back on purpose; `name` is `rust` for the toolchain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pin {
    pub name: String,
    pub reason: String,
    pub review: Option<Date>,
    pub until: Option<ReleaseCondition>,
    pub hold: Hold,
    pub at: Location,
}

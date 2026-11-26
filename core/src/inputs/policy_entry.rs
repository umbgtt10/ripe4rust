// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::policy_value::PolicyValue;
use crate::values::location::Location;
use alloc::string::String;

/// One key of the policy, its value and where it is written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyEntry {
    pub key: String,
    pub value: PolicyValue,
    pub at: Location,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::exposure_override::ExposureOverride;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::pins_and_policy::thresholds::Thresholds;
use crate::values::date::Date;
use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// The workspace's facts and thresholds the rules read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Policy {
    pub at: Option<Location>,
    pub gates: Vec<String>,
    pub published: Vec<String>,
    pub validate_beyond_gates: Vec<String>,
    pub freeze_until: Option<Date>,
    pub exposure: Vec<ExposureOverride>,
    pub thresholds: Thresholds,
    pub pins: Vec<Pin>,
}

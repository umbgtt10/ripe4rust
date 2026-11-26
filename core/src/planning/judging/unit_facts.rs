// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::footprint::Footprint;
use crate::planning::judging::line_standing::LineStanding;
use crate::planning::judging::pin_standing::PinStanding;
use crate::planning::judging::raise::Raise;
use crate::unit_kind::UnitKind;
use crate::values::date::Date;
use alloc::string::String;
use alloc::vec::Vec;

/// Everything the rules read of one unit. `name` is the unit as its reasons
/// write it; `rest` the count of moves, for the rest; `concerns` the
/// advisories and yanked releases it clears; `reaches` the targets needing
/// more than the gates it reaches; `pins` the pins holding what it moves,
/// `fully_held` whether they hold all of it; `freeze` the end of an open
/// freeze; `exposure_unknown_in` the published members using it whose public
/// API is not known; `footprint` `None` where the scan failed as a whole;
/// `gates` how many are declared.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitFacts {
    pub kind: UnitKind,
    pub name: String,
    pub rest: Option<u32>,
    pub concerns: Vec<String>,
    pub reaches: Vec<String>,
    pub pins: Vec<PinStanding>,
    pub fully_held: bool,
    pub freeze: Option<Date>,
    pub exposed_in: Vec<String>,
    pub exposure_unknown_in: Vec<String>,
    pub raises: Vec<Raise>,
    pub in_use_line: Option<LineStanding>,
    pub new_line: Option<LineStanding>,
    pub newer_active: bool,
    pub lines_behind: u32,
    pub footprint: Option<Footprint>,
    pub removes_copy: bool,
    pub held_back: u32,
    pub gates: u32,
    pub small_footprint: u32,
    pub moderate_footprint: u32,
}

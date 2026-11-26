// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::decision::Decision;
use crate::planning::judging::unit_facts::UnitFacts;

/// One rule of the table: a decision where it matches the unit, none where
/// it does not, so the first that matches decides.
pub trait Rule {
    fn decide(&self, facts: &UnitFacts) -> Option<Decision>;
}

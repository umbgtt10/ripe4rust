// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::edge::Edge;
use crate::values::version::Version;

/// The versions between two edges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interval {
    lower: Edge,
    upper: Edge,
}

impl Interval {
    pub fn new(lower: Edge, upper: Edge) -> Self {
        Self { lower, upper }
    }

    pub fn admits(&self, version: &Version) -> bool {
        let above = match &self.lower {
            Edge::Exclusive(lower) => version > lower,
            Edge::Inclusive(lower) => version >= lower,
            Edge::Open => true,
        };
        let below = match &self.upper {
            Edge::Exclusive(upper) => version < upper,
            Edge::Inclusive(upper) => version <= upper,
            Edge::Open => true,
        };
        above && below
    }
}

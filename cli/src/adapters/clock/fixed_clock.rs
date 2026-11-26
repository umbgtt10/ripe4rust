// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::traits::clock::Clock;
use ripe4rust_core::values::date::Date;

/// The day `--date` names, standing for today, so a plan is made again as
/// it was.
pub struct FixedClock {
    date: Date,
}

impl FixedClock {
    pub fn new(date: Date) -> Self {
        Self { date }
    }
}

impl Clock for FixedClock {
    fn today(&self) -> Date {
        self.date
    }
}

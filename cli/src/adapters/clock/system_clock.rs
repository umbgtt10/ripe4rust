// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::traits::clock::Clock;
use ripe4rust_core::values::date::Date;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const SECONDS_A_DAY: u64 = 86_400;

/// Today in UTC, from the system's clock -- the one place ripe4rust reads
/// it, and no test does -- counted from the epoch with the core's date
/// arithmetic, which the tests drive through `day_of`.
pub struct SystemClock;

impl SystemClock {
    pub fn day_of(&self, since_epoch: Duration) -> Date {
        Date::from_unix_days(
            i64::try_from(since_epoch.as_secs() / SECONDS_A_DAY).unwrap_or(i64::MAX),
        )
    }
}

impl Clock for SystemClock {
    fn today(&self) -> Date {
        self.day_of(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
        )
    }
}

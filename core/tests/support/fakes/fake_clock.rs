// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::traits::clock::Clock;
use ripe4rust_core::values::date::Date;

pub struct FakeClock {
    pub today: Date,
}

impl Clock for FakeClock {
    fn today(&self) -> Date {
        self.today
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::traits::pause::Pause;
use std::cell::RefCell;

/// A pause that waits on nothing and keeps every pause it was asked for.
pub struct CountingPause {
    pub pauses: RefCell<Vec<u64>>,
}

impl CountingPause {
    pub fn new() -> Self {
        Self {
            pauses: RefCell::new(Vec::new()),
        }
    }
}

impl Pause for CountingPause {
    fn pause(&self, seconds: u64) {
        self.pauses.borrow_mut().push(seconds);
    }
}

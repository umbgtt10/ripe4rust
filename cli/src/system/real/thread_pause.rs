// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::traits::pause::Pause;
use std::thread::sleep;
use std::time::Duration;

/// A pause that sleeps the thread.
pub struct ThreadPause;

impl Pause for ThreadPause {
    fn pause(&self, seconds: u64) {
        sleep(Duration::from_secs(seconds));
    }
}

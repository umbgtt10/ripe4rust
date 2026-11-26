// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::real::thread_pause::ThreadPause;
use cargo_ripe4rust::system::traits::pause::Pause;

#[test]
fn pause_of_no_seconds_returns_at_once() {
    // Arrange & Act & Assert
    ThreadPause.pause(0);
}

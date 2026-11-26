// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A wait between two requests to a service that asks for one at most so
/// often.
pub trait Pause {
    fn pause(&self, seconds: u64);
}

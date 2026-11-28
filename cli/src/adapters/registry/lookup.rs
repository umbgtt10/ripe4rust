// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// How a plan reads the day cache: online, a day's answer where it is
/// there and fetched where not; online and refreshed, every answer fetched
/// again and the day's copy overwritten; or offline, the cache alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lookup {
    Cached,
    Offline,
    Refreshed,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The numbers the rules compare with, each defaulted where the policy names
/// none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Thresholds {
    pub young_days: u32,
    pub young_releases: u32,
    pub toolchain_young_days: u32,
    pub stale_months: u32,
    pub small_footprint: u32,
    pub moderate_footprint: u32,
    pub hold_days: u32,
}

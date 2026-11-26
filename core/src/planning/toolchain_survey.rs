// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::release::Release;
use crate::values::location::Location;
use crate::values::version::Version;
use alloc::vec::Vec;

/// The toolchain as surveyed: the release the channel pins, where, the
/// toolchain in use, and the stable releases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolchainSurvey {
    pub pinned: Option<Version>,
    pub channel_at: Option<Location>,
    pub in_use: Version,
    pub releases: Vec<Release>,
}

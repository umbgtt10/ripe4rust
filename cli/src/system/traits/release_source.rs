// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;

/// The toolchain's stable releases, each with the day it was published.
pub trait ReleaseSource {
    fn releases(&self) -> Result<Vec<Release>, PortFailure>;
}

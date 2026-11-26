// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::scan::Scan;

/// The parse of the workspace's sources: footprint and exposure per crate.
pub trait SourceScan {
    fn scan(&self) -> Result<Scan, PortFailure>;
}

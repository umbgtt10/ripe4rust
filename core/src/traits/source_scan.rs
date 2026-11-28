// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::source_scan::scan::Scan;

/// The parse of the workspace's sources: footprint and exposure per crate,
/// or `None` where the scan failed as a whole -- not one source file could
/// be parsed --, so neither is known.
pub trait SourceScan {
    fn scan(&self) -> Result<Option<Scan>, PortFailure>;
}

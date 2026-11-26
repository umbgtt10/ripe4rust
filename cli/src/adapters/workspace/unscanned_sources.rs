// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::scan::Scan;
use ripe4rust_core::traits::source_scan::SourceScan;

/// The source scan until slice 4 parses the sources: footprint and exposure
/// not yet known, which the plan reads conservatively and says so.
pub struct UnscannedSources;

impl SourceScan for UnscannedSources {
    fn scan(&self) -> Result<Option<Scan>, PortFailure> {
        Ok(None)
    }
}

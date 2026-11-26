// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::scan::Scan;
use ripe4rust_core::traits::source_scan::SourceScan;

pub struct FakeScan {
    pub scan: Scan,
}

impl SourceScan for FakeScan {
    fn scan(&self) -> Result<Scan, PortFailure> {
        Ok(self.scan.clone())
    }
}

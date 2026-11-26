// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::unscanned_sources::UnscannedSources;
use ripe4rust_core::traits::source_scan::SourceScan;

#[test]
fn scan_of_sources_not_parsed_yet_is_not_known() {
    // Arrange & Act
    let scan = UnscannedSources.scan();

    // Assert
    assert_eq!(scan, Ok(None));
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::system::traits::release_source::ReleaseSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;

/// A release source answering what it was given: the releases, or why it
/// could not read them.
pub struct ListedReleases {
    pub releases: Result<Vec<Release>, PortFailure>,
}

impl ReleaseSource for ListedReleases {
    fn releases(&self) -> Result<Vec<Release>, PortFailure> {
        self.releases.clone()
    }
}

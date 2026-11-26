// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::traits::toolchain::Toolchain;
use ripe4rust_core::values::version::Version;

pub struct FakeToolchain {
    pub releases: Vec<Release>,
    pub rustc: Version,
    pub targets: Vec<String>,
}

impl Toolchain for FakeToolchain {
    fn releases(&self) -> Result<Vec<Release>, PortFailure> {
        Ok(self.releases.clone())
    }

    fn rustc(&self) -> Result<Version, PortFailure> {
        Ok(self.rustc.clone())
    }

    fn targets(&self) -> Result<Vec<String>, PortFailure> {
        Ok(self.targets.clone())
    }
}

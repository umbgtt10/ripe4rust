// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::advisories::Advisories;

pub struct FakeAdvisories {
    pub advisories: Vec<Advisory>,
    pub failure: Option<PortFailure>,
}

impl Advisories for FakeAdvisories {
    fn advisories(&self) -> Result<Vec<Advisory>, PortFailure> {
        match &self.failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.advisories.clone()),
        }
    }
}

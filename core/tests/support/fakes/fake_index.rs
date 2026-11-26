// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::traits::index::Index;
use ripe4rust_core::values::date::Date;

pub struct FakeIndex {
    pub read_on: Date,
    pub releases: Vec<(String, Vec<Release>)>,
    pub failure: Option<PortFailure>,
}

impl Index for FakeIndex {
    fn read_on(&self) -> Date {
        self.read_on
    }

    fn releases(&self, name: &str) -> Result<Vec<Release>, PortFailure> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        Ok(self
            .releases
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, releases)| releases.clone())
            .unwrap_or_default())
    }
}

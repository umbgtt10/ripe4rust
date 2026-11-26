// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::repositories::Repositories;

pub struct FakeRepositories {
    pub known: Vec<(String, String)>,
}

impl Repositories for FakeRepositories {
    fn repository(&self, name: &str) -> Result<Option<String>, PortFailure> {
        Ok(self
            .known
            .iter()
            .find(|(known, _)| known == name)
            .map(|(_, repository)| repository.clone()))
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::unreadable::Unreadable;
use ripe4rust_core::inputs::port_failure::PortFailure;
use serde_json::Value;
use serde_json::from_str;

/// The crates.io API's answer for a crate: the repository it names, `None`
/// where it names none or the answer is empty.
pub struct ApiReader;

impl ApiReader {
    pub fn repository(&self, name: &str, text: &str) -> Result<Option<String>, PortFailure> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        let answer = from_str::<Value>(text).map_err(|error| {
            Unreadable::new(&format!("reading the API's answer for `{name}`"))
                .failure(&error.to_string())
        })?;
        Ok(answer["crate"]["repository"]
            .as_str()
            .map(str::trim)
            .filter(|repository| !repository.is_empty())
            .map(String::from))
    }
}

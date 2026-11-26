// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::parse_failure::ParseFailure;

/// Output ripe4rust cannot read is a failure of its own, `internal`, naming
/// what it was reading: the tool printed something its reader does not
/// know, which is for ripe4rust to fix.
pub struct Unreadable {
    doing: String,
}

impl Unreadable {
    pub fn new(doing: &str) -> Self {
        Self {
            doing: String::from(doing),
        }
    }

    pub fn failure(&self, detail: &str) -> PortFailure {
        PortFailure::Internal {
            doing: self.doing.clone(),
            detail: String::from(detail),
        }
    }
    pub fn unparsed(&self, failure: &ParseFailure) -> PortFailure {
        self.failure(&format!("`{}` is not {}", failure.text, failure.expected))
    }
}

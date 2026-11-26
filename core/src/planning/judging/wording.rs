// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::format;
use alloc::string::String;

/// The few words the reasons are written with.
pub struct Wording;

impl Wording {
    pub fn count(&self, number: u32, noun: &str) -> String {
        if number == 1 {
            format!("1 {noun}")
        } else {
            format!("{number} {noun}s")
        }
    }

    pub fn list(&self, items: &[String]) -> String {
        items.join(", ")
    }
}

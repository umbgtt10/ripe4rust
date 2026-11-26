// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::footprint::Footprint;
use alloc::format;
use alloc::string::String;

/// The few words the reasons are written with: counts, lists and a
/// footprint, a floor said to be one.
pub struct Wording;

impl Wording {
    pub fn count(&self, number: u32, noun: &str) -> String {
        if number == 1 {
            format!("1 {noun}")
        } else {
            format!("{number} {noun}s")
        }
    }

    pub fn footprint(&self, footprint: Footprint) -> String {
        let counted = format!(
            "{} in {}",
            self.count(footprint.lines, "line"),
            self.count(footprint.files, "file")
        );
        match (footprint.complete, footprint.lines) {
            (true, _) => format!("footprint {counted}"),
            (false, 0) => String::from(
                "named nowhere the parse could read: code is included it cannot follow",
            ),
            (false, _) => {
                format!("footprint at least {counted}: code is included the parse cannot follow")
            }
        }
    }

    pub fn list(&self, items: &[String]) -> String {
        items.join(", ")
    }
}

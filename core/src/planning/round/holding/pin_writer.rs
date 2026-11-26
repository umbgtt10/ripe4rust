// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::planning::round::holding::held_crate::HeldCrate;
use crate::planning::round::holding::toml_text::TomlText;
use crate::values::date::Date;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

/// The line of a pin, as `[workspace.metadata.ripe4rust.pins]` holds one:
/// the reason given, the review date, and `hold = "all"` where every move is
/// held. Where the crate is pinned already, the line takes that pin's place
/// and keeps what it said: its reason first, its condition, the sooner of the
/// two reviews, and `hold = "all"` where either holds every move.
pub struct PinWriter {
    reason: String,
    review: Date,
}

impl PinWriter {
    pub fn new(reason: &str, review: Date) -> Self {
        Self {
            reason: String::from(reason),
            review,
        }
    }

    pub fn line(&self, held: &HeldCrate, pinned: Option<&Pin>) -> String {
        let reason = pinned.map_or_else(
            || self.reason.clone(),
            |pin| format!("{}; {}", pin.reason, self.reason),
        );
        let review = pinned
            .and_then(|pin| pin.review)
            .filter(|review| *review < self.review)
            .unwrap_or(self.review);
        let until = pinned
            .and_then(|pin| pin.until.as_ref())
            .map(|until| match until {
                ReleaseCondition::Release { name, requirement } => format!(
                    ", until = {{ crate = {}, version = {} }}",
                    TomlText.basic(name),
                    TomlText.basic(&requirement.to_string())
                ),
                ReleaseCondition::Words(words) => format!(", until = {}", TomlText.basic(words)),
            })
            .unwrap_or_default();
        let whole = held.hold == Hold::All || pinned.is_some_and(|pin| pin.hold == Hold::All);
        let scope = if whole { ", hold = \"all\"" } else { "" };
        format!(
            "{} = {{ reason = {}, review = \"{review}\"{until}{scope} }}",
            held.name,
            TomlText.basic(&reason)
        )
    }
}

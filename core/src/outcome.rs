// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What ripe4rust answers for an upgrade a workspace could take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Worth taking, but a person decides: presented with a suggestion.
    Ask,
    /// Safe to take without asking: applied, gated, committed or reverted.
    Ripe,
    /// Not yet: listed with its reason and what will release it.
    Ripening,
}

impl Outcome {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ask => "ask",
            Self::Ripe => "ripe",
            Self::Ripening => "ripening",
        }
    }
}

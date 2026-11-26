// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::planning::round::guiding::commands_guide::CommandsGuide;
use crate::planning::round::guiding::fields_guide::FieldsGuide;
use crate::planning::round::guiding::guide_text::GuideText;
use crate::planning::round::guiding::round_guide::RoundGuide;
use alloc::string::String;

/// `guide`: the round, the commands, every diagnostic code from the same
/// catalogue the diagnostics come from, and the document's fields, for the
/// release named.
pub struct Guide {
    version: String,
}

impl Guide {
    pub fn new(version: &str) -> Self {
        Self {
            version: String::from(version),
        }
    }

    pub fn text(&self) -> GuideText {
        GuideText {
            version: self.version.clone(),
            round: RoundGuide.steps(),
            commands: CommandsGuide.commands(),
            codes: DiagnosticCode::ALL.to_vec(),
            fields: FieldsGuide.fields(),
        }
    }
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::planning::round::guiding::command_guide::CommandGuide;
use crate::planning::round::guiding::field_guide::FieldGuide;
use alloc::string::String;
use alloc::vec::Vec;

/// What `guide` prints, versioned with the binary: the round, every
/// command, every diagnostic code -- each with what the catalogue says of it
/// -- and the fields of the documents of schema 1.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuideText {
    pub version: String,
    pub round: Vec<String>,
    pub commands: Vec<CommandGuide>,
    pub codes: Vec<DiagnosticCode>,
    pub fields: Vec<FieldGuide>,
}

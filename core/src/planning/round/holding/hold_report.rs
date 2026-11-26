// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::planning::round::holding::pin_line::PinLine;
use crate::values::location::Location;
use alloc::string::String;
use alloc::vec::Vec;

/// What `hold` prints: the unit, the section its pins belong in and the
/// line of that section, `None` where the manifest has none yet; the pins,
/// what to do with them, and the policy's warnings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoldReport {
    pub unit: String,
    pub section: String,
    pub at: Option<Location>,
    pub pins: Vec<PinLine>,
    pub action: String,
    pub diagnostics: Vec<Diagnostic>,
}

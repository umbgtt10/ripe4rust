// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::format::Format;
use ripe4rust_core::diagnostics::diagnostic::Diagnostic;

/// A command line that asks for nothing ripe4rust can do: the diagnostic
/// saying why, printed in the format it asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub command: String,
    pub format: Format,
    pub diagnostic: Diagnostic,
}

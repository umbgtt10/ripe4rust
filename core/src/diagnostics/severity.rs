// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// How much a diagnostic weighs: an error stops the command with exit 2; a
/// warning or a notice never changes the exit code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Notice,
    Warning,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Notice => "notice",
            Self::Warning => "warning",
        }
    }
}

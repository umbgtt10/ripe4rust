// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::severity::Severity;
use crate::inputs::port_failure::PortFailure;
use crate::planning::plan_failure::PlanFailure;

/// When a command stops, and with what: at the first error among what it
/// has found, or at a port that cannot answer, every diagnostic found so far
/// coming with it.
pub struct Stopping;

impl Stopping {
    pub fn at_errors(&self, found: &[Diagnostic]) -> Result<(), PlanFailure> {
        if found
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
        {
            Err(PlanFailure {
                diagnostics: found.to_vec(),
            })
        } else {
            Ok(())
        }
    }

    pub fn with(&self, found: &[Diagnostic], failure: &PortFailure) -> PlanFailure {
        let mut diagnostics = found.to_vec();
        diagnostics.push(failure.diagnostic());
        PlanFailure { diagnostics }
    }
}

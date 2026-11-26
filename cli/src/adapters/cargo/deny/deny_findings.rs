// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::deny::advisory_hit::AdvisoryHit;

/// What cargo-deny's log said: the advisories in force, the ignored ones
/// left out, and every error that kept it from checking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DenyFindings {
    pub hits: Vec<AdvisoryHit>,
    pub errors: Vec<String>,
}

// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// Why a program could not be started: it is not installed, or the system
/// refused, with what it said.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LaunchFailure {
    Failed { program: String, detail: String },
    Missing { program: String },
}

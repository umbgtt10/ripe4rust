// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// Why a file could not be read or written: it is not there, or the system
/// refused, with what it said.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileFailure {
    Failed { detail: String },
    Missing,
}

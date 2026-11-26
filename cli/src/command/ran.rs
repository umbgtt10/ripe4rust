// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What a run printed, its one document, and the code it exits with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ran {
    pub output: String,
    pub code: u8,
}

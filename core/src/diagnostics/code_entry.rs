// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What the catalogue says of a code, which `guide` prints: what it means,
/// and its remedy, the `do` every diagnostic of it says of its own case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodeEntry {
    pub means: &'static str,
    pub remedy: &'static str,
}

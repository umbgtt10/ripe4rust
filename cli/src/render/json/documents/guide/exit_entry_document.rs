// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// A code a command exits with, and what it means, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExitEntryDocument<'a> {
    pub code: u8,
    pub means: &'a str,
}

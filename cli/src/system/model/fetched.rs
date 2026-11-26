// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What a server answered: its HTTP status and the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fetched {
    pub status: u16,
    pub body: String,
}

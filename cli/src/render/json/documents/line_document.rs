// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// The line in use, its releases and the dates of its first and last.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LineDocument {
    pub line: String,
    pub first_release: Option<String>,
    pub last_release: Option<String>,
    pub releases: u32,
}

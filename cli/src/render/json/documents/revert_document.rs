// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// How to take a unit back: the files to restore, and the command that does.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RevertDocument<'a> {
    pub restore: &'a [String],
    pub run: &'a str,
}

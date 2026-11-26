// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// One field of a document of schema 1, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FieldEntryDocument<'a> {
    pub document: &'a str,
    pub field: &'a str,
    pub means: &'a str,
}

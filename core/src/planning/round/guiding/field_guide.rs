// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// One field of a document of schema 1: the document it is in -- `every`
/// for those all of them carry --, its path, an array's items as `[]`, and
/// what it holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldGuide {
    pub document: String,
    pub field: String,
    pub means: String,
}

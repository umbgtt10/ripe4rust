// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A git source moved as a whole: its URL and spec, and the short revs it
/// leaves and takes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitSource {
    pub source: String,
    pub from: String,
    pub to: String,
}

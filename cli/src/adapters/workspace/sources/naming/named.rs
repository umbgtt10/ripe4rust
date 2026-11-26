// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A line naming a dependency, by its package.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Named {
    pub package: String,
    pub line: u32,
}

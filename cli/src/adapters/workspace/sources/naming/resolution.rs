// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What the first name of a path stands for: a dependency in scope, by its
/// package; something of the workspace's own, of the standard library's or
/// out of scope; or nothing the file declares, which only a glob can have
/// brought in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    Crate(String),
    Local,
    Unknown,
}

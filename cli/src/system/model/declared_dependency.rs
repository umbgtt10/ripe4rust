// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// A dependency as a member's manifest declares it: the package, the name the
/// code calls it by where `package = ...` renames it, and where it comes from,
/// `None` for a path dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclaredDependency {
    pub name: String,
    pub rename: Option<String>,
    pub source: Option<String>,
}

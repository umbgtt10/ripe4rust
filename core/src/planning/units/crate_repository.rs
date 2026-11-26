// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

/// A direct dependency's repository, as the registry's API gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateRepository {
    pub name: String,
    pub repository: String,
}

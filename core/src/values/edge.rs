// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::version::Version;

/// One end of an interval of versions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edge {
    Exclusive(Version),
    Inclusive(Version),
    Open,
}

impl Edge {
    /// The same version as the edge of the other side: what ends a range
    /// inclusive starts the range past it exclusive.
    #[must_use]
    pub fn flipped(&self) -> Self {
        match self {
            Self::Exclusive(version) => Self::Inclusive(version.clone()),
            Self::Inclusive(version) => Self::Exclusive(version.clone()),
            Self::Open => Self::Open,
        }
    }
}

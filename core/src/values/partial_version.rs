// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::edge::Edge;
use crate::values::parse_failure::ParseFailure;
use crate::values::pre_release::PreRelease;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;

/// A version as a requirement or a `rust-version` writes it: `1`, `1.74`,
/// `1.2.3`, with a pre-release only after a patch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartialVersion {
    major: u64,
    minor: Option<u64>,
    patch: Option<u64>,
    pre: PreRelease,
}

impl PartialVersion {
    pub fn parse(text: &str) -> Result<Self, ParseFailure> {
        let failure = || ParseFailure {
            text: String::from(text),
            expected: "a version",
        };
        let without_build = text.split('+').next().unwrap_or_default();
        let (numbers, pre) = without_build
            .split_once('-')
            .map_or((without_build, None), |(numbers, pre)| {
                (numbers, Some(PreRelease::new(pre)))
            });
        let parts = numbers
            .split('.')
            .map(Self::number)
            .collect::<Option<Vec<u64>>>()
            .ok_or_else(failure)?;
        let well_formed = pre
            .as_ref()
            .is_none_or(|pre| parts.len() == 3 && pre.is_well_formed());
        let pre = pre.unwrap_or_else(|| PreRelease::new(""));
        match (parts.as_slice(), well_formed) {
            ([major], true) => Ok(Self::of(*major, None, None, pre)),
            ([major, minor], true) => Ok(Self::of(*major, Some(*minor), None, pre)),
            ([major, minor, patch], true) => Ok(Self::of(*major, Some(*minor), Some(*patch), pre)),
            _ => Err(failure()),
        }
    }

    fn of(major: u64, minor: Option<u64>, patch: Option<u64>, pre: PreRelease) -> Self {
        Self {
            major,
            minor,
            patch,
            pre,
        }
    }

    pub fn beyond(&self) -> Version {
        match (self.minor, self.patch) {
            (None, _) => Version::new(self.major.saturating_add(1), 0, 0),
            (Some(minor), None) => Version::new(self.major, minor.saturating_add(1), 0),
            (Some(minor), Some(patch)) => Version::new(self.major, minor, patch.saturating_add(1)),
        }
    }

    pub fn caret_ceiling(&self) -> Version {
        match (self.major, self.minor, self.patch) {
            (0, Some(0), Some(patch)) => Version::new(0, 0, patch.saturating_add(1)),
            (0, Some(minor), _) => Version::new(0, minor.saturating_add(1), 0),
            (major, _, _) => Version::new(major.saturating_add(1), 0, 0),
        }
    }

    pub fn floor(&self) -> Version {
        Version::new(self.major, self.minor.unwrap_or(0), self.patch.unwrap_or(0))
            .with_pre_release(self.pre.clone())
    }

    pub fn is_full(&self) -> bool {
        self.patch.is_some()
    }

    pub fn last(&self) -> Edge {
        if self.is_full() {
            Edge::Inclusive(self.floor())
        } else {
            Edge::Exclusive(self.beyond())
        }
    }

    pub fn names_pre_release_of(&self, version: &Version) -> bool {
        !self.pre.is_empty() && self.floor().release() == version.release()
    }

    pub fn tilde_ceiling(&self) -> Version {
        match self.minor {
            Some(minor) => Version::new(self.major, minor.saturating_add(1), 0),
            None => Version::new(self.major.saturating_add(1), 0, 0),
        }
    }

    fn number(part: &str) -> Option<u64> {
        if !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()) {
            part.parse().ok()
        } else {
            None
        }
    }
}

impl Display for PartialVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.major)?;
        if let Some(minor) = self.minor {
            write!(formatter, ".{minor}")?;
        }
        if let Some(patch) = self.patch {
            write!(formatter, ".{patch}")?;
        }
        if !self.pre.is_empty() {
            write!(formatter, "-{}", self.pre)?;
        }
        Ok(())
    }
}

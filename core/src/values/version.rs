// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::line::Line;
use crate::values::parse_failure::ParseFailure;
use crate::values::partial_version::PartialVersion;
use crate::values::pre_release::PreRelease;
use alloc::string::String;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;

/// A semver version; build metadata is read and dropped, as it orders
/// nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    pre: PreRelease,
}

impl Version {
    pub fn parse(text: &str) -> Result<Self, ParseFailure> {
        let partial = PartialVersion::parse(text)?;
        if partial.is_full() {
            Ok(partial.floor())
        } else {
            Err(ParseFailure {
                text: String::from(text),
                expected: "a version",
            })
        }
    }

    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: PreRelease::new(""),
        }
    }

    pub fn is_pre_release(&self) -> bool {
        !self.pre.is_empty()
    }

    pub fn line(&self) -> Line {
        if self.major == 0 {
            Line {
                major: 0,
                minor: Some(self.minor),
            }
        } else {
            Line {
                major: self.major,
                minor: None,
            }
        }
    }

    #[must_use]
    pub fn release(&self) -> Self {
        Self::new(self.major, self.minor, self.patch)
    }

    pub fn toolchain_line(&self) -> Line {
        Line {
            major: self.major,
            minor: Some(self.minor),
        }
    }

    #[must_use]
    pub fn with_pre_release(self, pre: PreRelease) -> Self {
        Self { pre, ..self }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.major, self.minor, self.patch)
            .cmp(&(other.major, other.minor, other.patch))
            .then_with(|| self.pre.cmp(&other.pre))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Version {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if self.is_pre_release() {
            write!(formatter, "-{}", self.pre)?;
        }
        Ok(())
    }
}
